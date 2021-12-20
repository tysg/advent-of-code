(ns advent-of-code.day-19
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clj-async-profiler.core :as prof]
   [clojure.zip :as z]))


(def two-d "--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1")

(def example "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14")

(defn parse-scanner [input]
  (let [lines (string/split-lines input)
        id (->> (first lines) 
                (re-matches #"--- scanner (\d+) ---") 
                second 
                (Integer/parseInt))
        points (->> (rest lines)
                    (map (fn [l] (mapv #(Integer/parseInt %)(string/split l #","))))
                    )]
    {id points}))

(defn parse-input [input]
  (into {} (map parse-scanner (string/split input #"\n\n"))))


(defn pairwise-offsets [points]
  (loop [dists #{} 
         [pt & others] (sort points)]
    (if (empty? others) 
      dists
      (let [pairwise (map (fn [p] (mapv - p pt)) others)]
        (recur (apply conj dists pairwise) others)))))


(defn orientations [[a b c]]
  (list [(- c)  (- b)  (- a)]  [(- c)  (- a)  b]  [(- c)  a  (- b)]  [(- c)  b  a] 
        [(- b)  (- c)  a]  [(- b)  (- a)  (- c)]  [(- b)  a  c]  [(- b)  c  (- a)] 
        [(- a)  (- c)  (- b)]  [(- a)  (- b)  c]  [(- a)  b  (- c)]  [(- a)  c  b] 
        [a  (- c)  b]  [a  (- b)  (- c)]  [a  b  c]  [a  c  (- b)] 
        [b  (- c)  (- a)]  [b  (- a)  c]  [b  a  (- c)]  [b  c  a] 
        [c  (- b)  a]  [c  (- a)  (- b)]  [c  a  b]  [c  b  (- a)]))
  

(defn points-orientations [points]
  (apply map vector (map orientations points)))


(defn get-offset [pts-a pts-b]
  (let [pairwise-a (pairwise-offsets pts-a)
        pairwise-b (pairwise-offsets pts-b)]
    (if (< (count (set/intersection (set pairwise-a) (set pairwise-b))) 66)
      nil
      (first (for [pa pts-a
                   pb pts-b
                   :let [offset (mapv - pa pb)
                         new-pts-b (map #(mapv + offset %) pts-b)]
                   :when (>= (count (set/intersection (set pts-a) (set new-pts-b))) 12)]
               offset)))))

(defn reduce-points [scanners]
  (loop [scanners scanners
         offsets {}
         stack [0] 
         visited #{}]
    (if (empty? stack)
      {:scanners scanners :offsets offsets}
      (let [top (peek stack)
            [scanners neighbours of] 
            (reduce
                (fn [[sc st of] [k v]]
                  (if (or (visited k) (st k) (= top k)) 
                    [(assoc sc k v) st of]
                    (if-let [res (->> v
                                  points-orientations
                                  (map #(vector % (get-offset (scanners top) %)))
                                  (filter #(some? (second %)))
                                  first)] 
                      (let [[oriented offset] res 
                            v-translated (map #(mapv + offset %) oriented)]
                        [(assoc sc k v-translated) (conj st k)  (assoc of k offset)])
                      ;; else
                      [(assoc sc k v) st of])))
                [{} #{} {}]
                scanners)]
        (recur scanners (into offsets of) (apply conj (pop stack) neighbours) (into visited neighbours))))))

(defn largest-dist [offsets]
  (apply max
         (for [p1 offsets
               p2 offsets]
           (reduce + (map #(Math/abs %) (map - p1 p2))))))

(comment
  (let [scanners (parse-input  (slurp "resources/day19.in"))
        a (scanners 0)
        b (scanners 9)]
    (remove nil? (for [pb (points-orientations b)]
                   (get-offset a pb))))
  ;; part 1
  (->> (slurp "resources/day19.in") ;example
       parse-input
       reduce-points
       :scanners
       (mapcat second)
       set
       count)
  

  ;; part 2
  (->> (slurp "resources/day19.in")
       ;example
       parse-input
       reduce-points
       :offsets
       vals
       largest-dist)
)  




     
