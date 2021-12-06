(ns advent-of-code.day-5
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.test :refer :all]))

(def example 
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2")

(def input (slurp "resources/day5.in"))

(defn parse-line [line]
  (let [[_ x1 y1 x2 y2] (re-matches #"(\d+),(\d+) -> (\d+),(\d+)" line)]
    [[(Integer/parseInt x1) (Integer/parseInt y1)] [(Integer/parseInt x2) (Integer/parseInt y2)]]))

(defn parse-input [input]
  (->> input 
       string/split-lines
       (map parse-line)))

(defn gen-points [[p1 p2]]
  (let [[p1 p2] (if (neg? (compare p1 p2)) [p1 p2] [p2 p1])
        [x1 y1] p1
        [x2 y2] p2]
    (cond
      ;; part 1
      (or (= x1 x2) (= y1 y2)) (for [x (range x1 (inc x2))
                                     y (range y1 (inc y2))] [x y])
      ;; part 2 - added diagonal
      (= (Math/abs (- x1 x2)) (Math/abs (- y1 y2))) 
      (let [grad (/ (- y2 y1) (- x2 x1))]
        (map vector (range x1 (inc x2)) (range y1 (+ y2 grad) grad))))))

(defn solve []
  (->> input
       ;;example
       parse-input
       (map gen-points)
       (remove empty?)
       (map #(apply hash-set %))
       (reduce (fn [[i u] x] [(set/union (set/intersection u x) i) (set/union u x)]) [(hash-set) (hash-set)])
       first
       count))

(deftest test-gen-points
  (is (gen-points [[0 3] [3 0]]) '([0 3] [1 2] [2 1] [3 0])))


(comment 
  (gen-points [[3 0] [0 3]])
  (gen-points [[1 1] [4 4]])
  (gen-points [[1 1] [1 4]])
)
(solve)
