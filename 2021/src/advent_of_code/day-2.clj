(ns advent-of-code.day-2
  (:require
   [clojure.string :as string]))


(defn parse-line [line]
  (let [[direc dist] (string/split line #" ")]
    [(keyword direc) (Integer/parseInt dist)]))

(defn execute [instrs]
  (loop [[curr-instr & rest] instrs
         depth 0
         x-pos  0
         ]
    (if (not curr-instr)
      [depth x-pos]
      (let [[direc dist] curr-instr]
        (case direc
          :forward (recur rest depth (+ x-pos dist))
          :down (recur rest (+ depth dist) x-pos)
          :up (recur rest (- depth dist) x-pos))))))

(defn execute-new [instrs]
  (loop [[curr-instr & rest] instrs
         depth 0
         x-pos  0
         aim 0]
    (if (not curr-instr)
      [depth x-pos]
      (let [[direc dist] curr-instr]
        (case direc
          :forward (recur rest
                          (+ depth (* aim dist))
                          (+ x-pos dist)
                          aim)
          :down (recur rest depth x-pos (+ aim dist))
          :up (recur rest depth x-pos (- aim dist)))))))

(->> (slurp "resources/day2a.in")
     (string/split-lines)
     (map parse-line)
     execute-new
     (apply *))