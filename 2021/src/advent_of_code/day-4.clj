(ns advent-of-code.day-4
  (:require
   [clojure.string :as string]))


(def input "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
")

(defn parse-grid [line]
  (mapv #(Integer/parseInt %) (string/split (string/trim line) #"\s+")))

(defn win? [grid]
  (let [rows (partition 5 (range 25))
        cols (map #(range % (+ 25 %) 5) (range 5))
        win-line? (fn [idx] (every? neg? (map #(get grid %) idx)))]
    (boolean (some true? (map win-line? (concat rows cols))))))

(defn move-grid [grid val]
  (let [idx (.indexOf grid val)]
    (if (neg? idx)
      grid
      (update grid (.indexOf grid val) #(- % 100)))))

(defn pivot-by [pred coll]
  (let [result (group-by pred coll)]
    (vector (get result true []) (get result false []))))

(defn sum-unmarked [grid]
  (reduce + (filter pos? grid)))

(defn move [state]
  (let [{:keys [winning-grids winning-moves grids moves]} state]
    (if (or (empty? grids) (empty? moves))
      state
      (let [[wins next-grids] (pivot-by win? (map #(move-grid % (first moves)) grids))]
        (if (empty? wins)
          {:winning-grids winning-grids 
           :winning-moves winning-moves 
           :grids next-grids 
           :moves (rest moves)}
          {:winning-grids (cons (first wins) winning-grids)
           :winning-moves (cons (first moves) winning-moves)
           :grids next-grids
           :moves (rest moves)})))))

(let [input (slurp "resources/day4.in")
      lines (string/split input #"\n\n")
      moves (map #(Integer/parseInt %) (string/split  (first lines) #","))
      grids (map parse-grid (rest lines))]
  
  (let [winning 
        (->> {:winning-grids [] :winning-moves [] :grids grids :moves moves}
             (iterate move)
             ;; (drop-while #(empty? (:winning-grids %)))
             (drop-while #((complement empty?) (:grids %)))
             first
)]
    (* 
     (first (:winning-moves winning)) 
     (sum-unmarked (first (:winning-grids winning))))
))
