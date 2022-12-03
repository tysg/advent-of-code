(ns advent-of-code.22-day-3
  (:require
   [clojure.core.match :refer [match]]
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

;; for each type, all items must go to the same
;; compartment.
;; per rucksack, failed to cat 1 item

(def example "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw")

(defn common-item [racksack]
  (let [[l-comp r-comp]
        (partition (/ (count racksack) 2) racksack)]
    (first (filter (set l-comp) r-comp))))

(defn priority [c]
  (if (java.lang.Character/isUpperCase c)
    (- (int c) 38)
    (- (int c) 96)))

(comment
  "part 1"
  (->> (string/split-lines (slurp "resources/22-day3.in"))
       (map vec)
       (map common-item)
       (map priority)
       (reduce +)))

(defn badge [group]
  (let [[l m r] (map frequencies group)]
    (first
     (filter #(and (contains? l %) (contains? m %) (contains? r %))
             (vec "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")))))

(->> (string/split-lines (slurp "resources/22-day3.in"))
     (partition 3)
     (map badge)
     (map priority)
     (reduce +))
