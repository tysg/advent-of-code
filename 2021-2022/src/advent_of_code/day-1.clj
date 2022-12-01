(ns advent-of-code.day-1
  (:require [clojure.string :as string]))

(defn parse-int-series [input]
  (map #(Integer/parseInt %)
       (string/split input #"\n")))

(defn count-increase [l]
  (count (filter #(apply < %) (partition 2 1 l))))

(defn into-three-measurement-window [l]
  (map #(reduce + %) (partition 3 1 l)))


(let [input (slurp "resources/day1a.in")
      series (parse-int-series input)]
  ;; (count-increase series) ; part 1: 1557
  (count-increase (into-three-measurement-window series)) ; part 2: 1608
  )

