(ns advent-of-code.day-4
  (:require
   [clojure.string :as string]))

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
      (update grid idx #(- % 100)))))

(defn pivot-by [pred coll]
  (let [result (group-by pred coll)]
    (vector (get result true []) (get result false []))))

(defn sum-unmarked [grid] (reduce + (filter pos? grid)))

(defn move [state]
  (let [{:keys [winning-grids winning-moves grids moves]} state]
    (if (or (empty? grids) (empty? moves))
      state
      (let [[wins next-grids] (pivot-by win? (map #(move-grid % (first moves)) grids))]
        {:winning-grids (if (empty? wins) winning-grids (conj winning-grids (first wins))) 
         :winning-moves (if (empty? wins) winning-grids (conj winning-moves (first moves))) 
         :grids next-grids 
         :moves (rest moves)}))))

(let [input (slurp "resources/day4.in")
      lines (string/split input #"\n\n")
      moves (map #(Integer/parseInt %) (string/split  (first lines) #","))
      grids (map parse-grid (rest lines))]
  
  (let [winning
        (->> {:winning-grids [] :winning-moves [] :grids grids :moves moves}
             (iterate move)
             (take 1000)
             ;; (drop-while #(empty? (:winning-grids %)))
             (drop-while #((complement empty?) (:grids %)))
             first)]
    (*
     (peek (:winning-moves winning))
     (sum-unmarked (peek (:winning-grids winning))))))
