(ns advent-of-code.day-20
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clj-async-profiler.core :as prof]
   [clojure.zip :as z]))

(def example "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###")

(def w 512)

(defn parse-input [input]
  (let [[lookup grid-lines] (string/split input #"\n\n")
        lookup (string/trim lookup)
        stride (.indexOf grid-lines "\n")
        padding (- w stride)]
    {:lookup (string/trim lookup)
     :grid (-> grid-lines 
               (string/escape {\. "\0"
                               \# "\1"
                               \newline (String. (byte-array padding))})
               .getBytes
               (java.util.Arrays/copyOf (* w w)))}))

(def direc (for [dr [(- w) 0 w]
                 dc [-1 0 1]]
             (+ dr dc)))


(defn process [input]
  (let [{^String lookup :lookup grid :grid} (parse-input input)
        next-grid 
        (fn ^bytes [^bytes grid]
          (let [res (byte-array (* w w))]
            (loop [pos 0]
              (if (= pos (* w w))
                res
                (let [n (Integer/parseInt (->> direc
                                               (map #(aget grid (mod (+ pos %) (* w w))))
                                               (string/join))
                                          2)
                      y (case (.charAt lookup n)
                          \. 0
                          \# 1)]
                  (aset-byte res pos y)
                  (recur (inc pos)))))))]
    (as-> grid $
         (iterate next-grid $)
         (nth $ 50)
         (reduce + $)
         )))
(process ; (slurp "resources/day20.in")
 example
 )



