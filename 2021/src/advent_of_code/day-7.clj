(ns advent-of-code.day-7
  (:require
   [clojure.string :as string]
   [clojure.set :as set]))

(def example "16,1,2,0,4,2,7,1,2,14")
(def input (slurp "resources/day7.in"))

(defn parse-input [input] 
  (map #(Long/parseLong %) (string/split (string/trim input) #"," )))

(defn euclidean-dist [^long a ^long b] (Math/abs (- a b))) 

(defn summation [n] (quot (* n (inc n)) 2)) 

(defn summation-dist [a b] (summation (euclidean-dist a b)))

(defn fuel-use [positions target metric] 
  (reduce + (map #(metric target %) positions)))

(defn solve 
  ([] (solve example))
  ([input] 
   (let [numbers (parse-input input)
         nmin (apply min numbers)
         nmax (apply max numbers)]
     (apply min (map #(fuel-use numbers % summation-dist) (range nmin (inc nmax)))))))

(comment
  (time (solve input)))
