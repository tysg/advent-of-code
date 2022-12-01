(ns advent-of-code.22-day-1
  (:require
   [clojure.core.match :refer [match]]
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

(def example "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")

(defn calories [in]
  (->>
   (string/split in #"\n\n")
   (map (fn [s]
          (->> s
               (string/split-lines)
               (map #(Integer/parseInt % ) )
               (reduce + ))))))

(comment "part 1"
         (apply max (calories (slurp "resources/22-day1.in"))))

(reduce + (take 3 (reverse (sort (calories (slurp "resources/22-day1.in"))))))
