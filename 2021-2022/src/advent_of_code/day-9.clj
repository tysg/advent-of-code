(ns advent-of-code.day-9
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.test :refer [is run-tests]]))

(def example 
"2199943210
3987894921
9856789892
8767896789
9899965678")

(def input (slurp "resources/day9.in"))

(def small 
"12
34")


(defn parse-input [input]
   (let [stride (.indexOf input "\n")]
     (mapv vec (partition stride (mapv #(Integer/parseInt %) (re-seq #"\d" input))))))


(def direc '([0 1] [1 0] [0 -1] [-1 0]))

(defn troughs [grid]
  (->>
   (for [r (range (count grid))
         c (range (count (first grid)))
         :let [v (get-in grid [r c])]
         :when (every? 
                (fn [[dr dc]] 
                  (let [nv (get-in grid [(+ r dr) (+ c dc)])]
                    (if (nil? nv) true (< v nv)))) 
                direc)]
     [r c])))


(defn flood-fill [grid troughs]
  (reduce
   (fn [[grid floods] trough]
     (loop [grid grid
            curr-size 0
            stack (vector trough)]
       (if (empty? stack)
         [grid (conj floods curr-size)]
         (let [[r c] (peek stack)
               val (get-in grid [r c])
               stack (pop stack)
               neighbours (->> direc 
                               (map (fn [[dr dc]] (vector (+ r dr) (+ c dc)))) ; list of neighbours idx
                               (filter (fn [[nr nc]] 
                                         (let [v (get-in grid [nr nc])]
                                           (and 
                                            (some? v) 
                                            (not= 9 v) 
                                            (> v val))))))]
           (recur (update-in grid [r c] (constantly 9))
                  (if (not= val 9) (inc curr-size) curr-size)
                  (into stack neighbours))))))
   [grid []]
   troughs))

(defn part-1 [input]
  (let [grid (parse-input input)]
    (->> (troughs grid)
         (map #(inc (get-in grid %)))
         (reduce +))))


(defn part-2 [input]
  (let [grid  (parse-input input)
        [_ flood] (flood-fill grid (troughs grid))]
    (reduce * (take 3 (reverse (sort flood ))))))

;(part-2 input)

