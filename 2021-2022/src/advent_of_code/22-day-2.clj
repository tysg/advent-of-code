(ns advent-of-code.22-day-2
  (:require
   [clojure.core.match :refer [match]]
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

(def example "A Y
B X
C Z")


(comment
  (reduce + (take 3 (reverse (sort (calories (slurp "resources/22-day1.in")))))))

(defn win? [self other]
  (boolean (some #(= % [self other]) [[:rock :scissors] [:paper :rock] [:scissors :paper]])))

(def code->hand
  {
   "A" :rock
   "B" :paper
   "C" :scissors
   "X" :rock
   "Y" :paper
   "Z" :scissors
   })

(defn score [other self]
  (let [win-score (if (= self other) 3 (if (win? self other) 6 0))
        hand-score (case self
                     :rock 1
                     :paper 2
                     :scissors 3)]
    (+ win-score hand-score)))

(comment
  (score :paper :rock)
  (score :rock :paper)
  (score :scissors :scissors))

(defn total-score [s]
  (reduce +
          (map (fn [line]
                 (->> (string/split line #" ")
                      (map code->hand)
                      (apply score)))
               (string/split-lines s))))
(total-score (slurp "resources/22-day2.in"))
(total-score example)

(defn hand-for-result [opp res]
  (case res
    :draw opp
    :win (first (filter #(win? % opp) [:rock :paper :scissors]))
    :lose(first (filter #(win? opp %) [:rock :paper :scissors]))
))

(def decode
  {
   "A" :rock
   "B" :paper
   "C" :scissors
   "X" :lose
   "Y" :draw
   "Z" :win
   })

(defn total-score-2 [s]
  (reduce +
          (map (fn [line]
                 (let [[opp res] (->> (string/split line #" ") (map decode))
                       self (hand-for-result opp res)]
                   (score opp self)
                   ))
               (string/split-lines s))))

(total-score-2 (slurp "resources/22-day2.in"))
(total-score-2 example)
