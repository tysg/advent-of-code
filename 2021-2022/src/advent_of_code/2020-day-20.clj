(ns advent-of-code.2020-day-20
  (:require [clojure.string :as string]))


(defn border [tile idxs]
  (string/join (map  #(.charAt tile %) idxs)))

(defn parse-tile [tile] 
  (let [[_, id, data] (re-matches #"Tile (\d+):([\.#\n]+)" tile)
        id (Integer/parseInt id)
        top (range 1 11)
        bottom (range 100 110)
        left (range 1 110 11)
        right (range 10 120 11)]
    
    {:id id
     :left (border data left)
     :right (border data right)
     :top (border data top)
     :bottom (border data bottom)
     :payload (map #(border data (range (+ % 13) (+ % 21))) (range 0 88 11))}
    ))


(defn rotate [{:keys [id left right top bottom payload]}]
  {:id id
   :left bottom
   :top (string/reverse left)
   :right top
   :bottom (string/reverse right)
  ;;  :payload 
   ;; TODO
   })


(def sample-tile "
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###")

(let [input (slurp "resources/2020_day20_ex.in")
      tiles (string/split input #"\n\n")]
   (->> tiles
        first
        parse-tile))
