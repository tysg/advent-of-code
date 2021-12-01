(ns advent-of-code.day-1
  (:require
   [advent-of-code.core :as core]))


(defn count-increase [l]
  (let [[head & rest] l]
    (if (empty? rest)
      0
      (+
       (if (< head (first rest)) 1 0)
       (count-increase rest)))))


(defn into-three-measurement-window [l]
  (map #(reduce + %) (partition 3 1 l)))


(let [input (slurp "resources/day1a.in")
      series (core/parse-int-series input)]
  ;; (count-increase series) ; part 1
  (count-increase (into-three-measurement-window series)) ; part 2
  )
