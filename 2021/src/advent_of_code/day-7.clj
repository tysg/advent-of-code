(ns advent-of-code.day-7
  (:require
   [clojure.string :as string]
   [clojure.set :as set]))

(def example "16,1,2,0,4,2,7,1,2,14")
(def input (slurp "resources/day7.in"))

(defn parse-input [input] 
  (map #(Integer/parseInt %) (string/split (string/trim input) #"," )))

(defn fuel-use [positions target]
  (reduce + (map #(Math/abs (- % target)) positions)))

(defn summation [n] (/ (* n (inc n)) 2)) 

(defn fuel-use-new [positions target]
  (reduce + (map #(summation (Math/abs (- % target))) positions)))

(defn solve 
  ([] (solve example))
  ([input] 
   (let [numbers (parse-input input)]
     (apply min (map #(fuel-use-new numbers %) (range (apply min numbers) (inc (apply max numbers))))))))

(comment
  (solve input))
