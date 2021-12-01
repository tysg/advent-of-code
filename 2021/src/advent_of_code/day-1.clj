
(ns advent-of-code.day-1
  (:require
   [advent-of-code.core :as core]))

(def input (slurp "resources/day1a.in"))

(defn count-increase [l]
  (let [[head & rest] l]
    (if (empty? rest)
      0
      (+
       (if (< head (first rest)) 1 0)
       (count-increase rest)))))

(def input-series (core/parse-int-series input))

(def part-1 (count-increase input-series))

(defn into-three-measurement-window [l]
  (if (< (count l) 3)
    '()
    (cons (reduce + (take 3 l))
          (into-three-measurement-window (rest l)))))


(def part-2 (count-increase (into-three-measurement-window input-series)))


