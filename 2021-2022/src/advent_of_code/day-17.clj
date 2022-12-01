(ns advent-of-code.day-17
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]))

(def example "target area: x=20..30, y=-10..-5")

(def input "target area: x=70..96, y=-179..-124")

(defn next-pos [[vx vy x y]]
  [(if (pos? vx) (dec vx) vx) (dec vy) (+ x vx) (+ y vy)])

(defn trajectory [[x1 x2] [y1 y2] [vx vy]]
  (take-while 
   (fn [[_ _ x y]] (and (<= 0 x x2) (>= y y1))) 
   (iterate next-pos [vx vy 0 0])))


(defn hit? [[x1 x2] [y1 y2] trajectory]
  (let [[_ _ x y] (last trajectory)] (and (<= x1 x x2) (<= y1 y y2))))

(defn highest-y [trajectory] (apply max (map #(nth % 3) trajectory)))

(defn solve [input] 
  (let [[_ x1 x2 y1 y2] (re-matches #"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)" input)
        x-bound [(Integer/parseInt x1) (Integer/parseInt x2)]
        y-bound [(Integer/parseInt y1) (Integer/parseInt y2)]]

    (->> (for [vx (range -500 500)
               vy (range -500 500)
               :let [t (trajectory x-bound y-bound [vx vy])]
               :when (hit? x-bound y-bound t)]
           (highest-y t))
         ;; part 1
         ;(apply max)
         ;; part 2
         count
)))

(solve input)

