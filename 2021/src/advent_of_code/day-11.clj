(ns advent-of-code.day-11
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

(def small "11111
19991
19191
19991
11111")

(def example "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")

(def input "3322874652
5636588857
7755117548
5854121833
2856682477
3124873812
1541372254
8634383236
2424323348
2265635842
")

(defn print-grid [grid] (clojure.pprint/pprint (partition (+ 2 width) grid)))

(def width 10)

(defn parse-grid [input]
  (let [width (.indexOf input "\n")
        numbers (map #(Integer/parseInt %) (re-seq #"\d" input))]
    (vec (flatten (concat 
                   (list (repeat (+ width 2) -1)) 
                   (map #(concat (cons -1 %) '(-1)) (partition width numbers))
                   (list (repeat (+ width 2) -1)))))))

(defn evolve [grid]
   (let [width (+ 2 width)
         direc [(- -1 width)  (- width) (- 1 width)
                -1                      1
                (dec width)   width     (inc width)]]
     (loop [grid (mapv #(if (neg? %) % (mod (inc %) 10)) grid)
            [lighted & rest] (filter #(zero? (nth grid %)) (range (count grid)))]
       (if (nil? lighted)
         grid
         (let [neighbours (map #(+ % lighted) direc)
               newly-lighted (filter #(= 9 (nth grid %)) neighbours)
               next-grid (reduce 
                          (fn [g idx] (update g idx #(if (pos? %) (mod (inc %) 10) %)))
                          grid
                          neighbours)]
           (recur next-grid (concat rest newly-lighted)))))))

(defn part-1 [input]
  (->> input parse-grid (iterate evolve) (drop 1) (take 100) (map #(count (filter zero? %))) (reduce +)))

(defn part-2 [input]
  (->> input parse-grid (iterate evolve) (take 10000) (take-while #(some pos? %)) count))


(part-1 input)
