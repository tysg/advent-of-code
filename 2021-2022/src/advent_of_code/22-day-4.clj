(ns advent-of-code.22-day-4
  (:require
   [clojure.core.match :refer [match]]
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

(def example
  "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8")

(defn fully-contain? [[l-s l-e] [r-s r-e]]
  (let [overlap [(max l-s r-s) (min l-e r-e)]]
    (or (= overlap [l-s l-e])
        (= overlap [r-s r-e]))))

(defn overlap? [[l-s l-e] [r-s r-e]]
  (<= (max l-s r-s) (min l-e r-e)) )

(defn parse-line [line]
  (let [[_ l-s l-e r-s r-e] (re-matches #"(\d+)-(\d+),(\d+)-(\d+)" line)]
    [[(Integer/parseInt l-s) (Integer/parseInt l-e)]
     [(Integer/parseInt r-s) (Integer/parseInt r-e)]]))

(->> (slurp "resources/22-day4.in")
     (string/split-lines)
     (map parse-line)
     (filter #(apply overlap? %))
     count)
