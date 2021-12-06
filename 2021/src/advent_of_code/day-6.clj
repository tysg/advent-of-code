(ns advent-of-code.day-6
  (:require
   [clojure.string :as string]
   [clojure.set :as set]))

(def example "3,4,3,1,2")

(def input (slurp "resources/day6.in"))

(defn parse-input [input] (string/split (string/trim input) #","))

(defn next-timer [timer] (if (pos? timer) (dec timer) 6))

(defn next-day [state]
  (reduce
   (fn [acc [timer freq]]
     (if (zero? timer)
       (update (assoc acc 8 freq) 6 #(+ freq (if (nil? %) 0 %)))
       (update acc (next-timer timer) #(+ freq (if (nil? %) 0 %)))))
   {}
   state))


(defn process 
  ([] (process example))
  ([input] (->> (parse-input input)
            (map #(Integer/parseInt %))
            frequencies
            (iterate next-day)
            (drop 256)
            first
            vals
            (reduce +)
            )))

(comment
  (next-day {7 1, 1 2, 0 3, 6 1, 8 1})
  (process input)
)
