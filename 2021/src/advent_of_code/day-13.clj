(ns advent-of-code.day-13
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]))


(def example "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5")

(def input (slurp "resources/day13.in"))

(defn parse-input [input]
  (let [[points instrs] (string/split input #"\n\n")]
       {
        :pts (->> points
                  string/split-lines
                  (map #(let [[_ x y] (re-matches #"(\d+),(\d+)" %)]
                          [(Integer/parseInt x) (Integer/parseInt y)]))
                  set)
        :instrs (->> instrs 
                     string/split-lines 
                     (map #(let [[_ axis val] (re-matches #"fold along (x|y)=(\d+)" %)]
                             [(keyword axis) (Integer/parseInt val)])))
}))



(defn fold [pts instr]
  (let [[axis val] instr]
    (reduce 
     (fn [pts [x y]]
       (conj pts
             (case axis
               :x (if (> x val) [(- (* 2 val) x) y] [x y])
               :y (if (> y val) [x (- (* 2 val) y)] [x y]))))
     #{}
     pts)))


(defn print-grid [pts]
  (let [width (inc (apply max (map first pts)))
        height  (inc (apply max (map second pts)))
        arr (make-array Integer/TYPE height width)]
    (doseq [[x y] pts] (aset-int arr y x 1))
    (string/replace (string/join "\n" (map #(reduce str (aget arr %)) (range height)))
                    #"1|0"
                    {"1" "X"
                     "0" " "})))


(let [{:keys [pts instrs]} (parse-input input)]
  (println (print-grid (reduce fold pts instrs))))
