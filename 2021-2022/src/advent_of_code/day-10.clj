(ns advent-of-code.day-10
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.test :refer [is run-tests]]))


(def score {\) 3 \] 57 \} 1197 \> 25137})

(def example "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]")

(def input (slurp "resources/day10.in"))


(defn first-incorrect [line]
  (loop [s []
         [c & rest] line]
    (cond 
      (nil? c) s
      (#{\( \{ \< \[} c) (recur (conj s c) rest)
      :else (if (case c
                  \) (= (peek s) \()
                  \} (= (peek s) \{)
                  \] (= (peek s) \[)
                  \> (= (peek s) \<))
              (recur (pop s) rest)
              c))))

(defn part-1 [input]
  (->> input
       string/split-lines
       (map first-incorrect)
       (filter char?)
       (map score)
       (reduce +)))

(defn completion-score [stack]
  (->> stack
       reverse
       (reduce 
        (fn [acc c] (+ ({\( 1 \[ 2 \{ 3 \< 4} c) (* 5 acc))) 
        0)))


(defn part-2 [input]
  (let [scores (->> input
                    string/split-lines
                    (map first-incorrect)
                    (filter vector?)
                    (map completion-score)
                    sort)]
       (nth scores (quot (count scores) 2))))

(part-2 input)
