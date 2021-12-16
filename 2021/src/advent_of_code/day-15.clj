
(ns advent-of-code.day-15
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]))

(def example "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581")
 
(defn idx-into [width [r c]] (+ c (* r width)))

(defn inc-number [n x] 
  (loop [n n x x]
    (if (zero? n) 
      x
      (if (= x 9) 
        (recur (dec n) 1)
        (recur (dec n) (inc x))))))



(defn full-map [numbers width]
  (let [row  (fn [numbers] (->> (range 5)
                                (map #(map (partial inc-number %) numbers))
                                (map #(partition width %))
                                (apply map concat)
                                (apply concat)))]
    (->> (range 5) (map #(map (partial inc-number %) numbers)) (map row) (apply concat))))


(defn shortest-path [input]
  (let [width (.indexOf input "\n")
        numbers (mapv #(Integer/parseInt %) (re-seq #"\d" input))
        ;; part 2
        numbers (vec (full-map numbers width))
        width (* 5 width)
        ]
    
    (loop [q (priority-map [0 0] 0)
           r {}]
      (if-let [[v d] (peek q)]
        (let [direc '([0 1] [1 0] [-1 0] [0 -1])
              neighbours (->> (map #(mapv + v %) direc)
                              (filter (fn [[r c]] (and (<= 0 r (dec width)) (<= 0 c (dec width)))))
                              (remove r)
                              (map (fn [v] {v (+ d (nth numbers (idx-into width v)))}))
                              (apply merge))
              ]
          (recur (merge-with min (pop q ) neighbours) (assoc r v d)))
        r))))



((shortest-path (slurp "resources/day15.in")) [499 499])
