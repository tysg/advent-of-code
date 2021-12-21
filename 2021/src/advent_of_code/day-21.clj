(ns advent-of-code.day-21)

(defn next-pos [pos delta]
  (inc (mod (+ (dec pos) delta) 10)))

(def other-player {:a :b :b :a})

(defn play [[last-game dice]]
  (let [rolls (take 3 dice)
        move (fn [[score pos]] 
               (let [new-pos (next-pos pos (reduce + rolls) )]
                 [(+ score new-pos) new-pos]))
        this-player (other-player (:player last-game)) ]

    [(assoc last-game 
            this-player
            (move (this-player last-game))

            :round
            (inc (:round last-game))

            :player
            this-player)
     (drop 3 dice)]))

(defn winning-game [init-game dice goal]
  (->> [init-game dice]
       (iterate play)
       (drop-while #(and (< (get-in % [0 :a 0]) goal)
                         (< (get-in % [0 :b 0]) goal)))
       ffirst))

; game e.g. {:a [0 4] :b [0 8] :round 0 :player :a}
;           score  position
(defn part-1 []
  (let [winning-game (winning-game 
                      {:a [0 6] :b [0 2] :round 0 :player :b}
                      (cycle (range 1 101))   
                      1000)]
    (* (first (get winning-game (other-player (:player winning-game))))
       (* 3 (:round winning-game)))))

(def worlds-won
  (memoize 
   (fn [game]
     (if (>= (first (game (:player game))) 21)
       {(:player game) 1}
       (->> (for [d1 '(1 2 3)
                  d2 '(1 2 3)
                  d3 '(1 2 3)]
              [d1 d2 d3])
            (map #(let [[next-game _] (play [game %])] next-game))
            (map worlds-won)
            (apply merge-with +))))))


(part-1)
;(worlds-won {:a [0 6] :b [0 2] :round 0 :player :b})










