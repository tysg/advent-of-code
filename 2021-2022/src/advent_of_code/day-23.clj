(ns advent-of-code.day-23
  (:require 
   [clojure.test :refer [deftest is]]
   [clojure.data.priority-map :refer [priority-map]]))


"
#############
#01.3.5.7.910#
###A#B#C#D###
  #A#B#C#D#
  #########
"

(def example-state {:as [\a \b] :bs [\d \c] :cs [\c \b] :ds [\a \d] :corridor {}})

(def energy-per-step {\a 1 \b 10 \c 100 \d 1000})

(def room-pos {:as 2 :bs 4 :cs 6 :ds 8})

(def room-size 4)

(defn move-amphipod [state room corridor-pos direction]
  (let [amphipod (case direction
                   :to-corridor (peek (room state))
                   :to-room ((:corridor state) corridor-pos))
        room-pos (room-pos room)
        steps (+ (Math/abs (- room-pos corridor-pos))
                 (case direction
                   :to-room (- room-size (count (room state)))
                   :to-corridor (- (inc room-size) (count (room state)))))
        new-cost (* steps (energy-per-step amphipod))]

    {:state (case direction
              :to-corridor
              (-> state 
                  (update-in [room] pop) 
                  (assoc-in [:corridor corridor-pos] amphipod))
              :to-room
              (-> state 
                  (update-in [room] #(conj % amphipod)) 
                  (update-in [:corridor] #(dissoc % corridor-pos))))
     :cost new-cost}))

(defn goal? [state] (and (every? #(= \a %) (:as state))
                         (every? #(= \b %) (:bs state))
                         (every? #(= \c %) (:cs state))
                         (every? #(= \d %) (:ds state))
                         (= room-size 
                            (count (:as state)) 
                            (count (:bs state)) 
                            (count (:cs state)) 
                            (count (:ds state)))))



(defn moves 
  "returns a list of next states"
  [state]
  (let [corridor (:corridor state)
        corridor-to-room (for [[corridor-pos amphipod] corridor
                               :let [target-room ({\a :as \b :bs \c :cs \d :ds} amphipod)
                                     target-idx (room-pos target-room)]
                               :when (and 
                                      (every? #( = % amphipod) (state target-room))
                                      (< (count (state target-room)) room-size)
                                      (every? #(not (contains? corridor %))
                                              (if (< corridor-pos target-idx)
                                        ; going right
                                                (range (inc corridor-pos) target-idx)
                                        ; going left
                                                (range (dec corridor-pos) target-idx -1))))]
                           (move-amphipod state target-room corridor-pos :to-room))]
    (if (not (empty? corridor-to-room))
      corridor-to-room
      ;; move a piece to all open space in the corridor
      (apply concat
             (for [room (->> [:as :bs :cs :ds]
                             (remove #(empty? (% state))))
                   :let [idx (room-pos room)
                         [left-range right-range] (split-with #(< % idx) [0 1 3 5 7 9 10])
                         left-idx (take-while #(not (contains? corridor % )) (reverse left-range))
                         right-idx (take-while #(not (contains? corridor %)) right-range)]]
               (map #(move-amphipod state room % :to-corridor) (concat left-idx right-idx)))))))


(defn bfs [state]
  (loop [q (priority-map state 0)]
    (if-let [[state cost] (peek q)]
      (if (goal? state)
        cost
        (let [neighbours (->> (moves state)
                              (map (fn [res] {(:state res) (+ (:cost res) cost)}))
                              (apply merge))]
          (recur (merge-with min (pop q) neighbours)))))))

(def one-step {:as [\a] :bs [\b \b] :cs [\c \c] :ds [\d \d] :corridor {9 \a}})
(def two-step {:as [\a] :bs [\b \b] :cs [\c \c] :ds [] :corridor {5 \d 7\d 9 \a}})
(def three-step {:as [\a] :bs [\b \b] :cs [\c \c] :ds [\a \d] :corridor {5 \d}})

(def input {:as [\b \c] :bs [\a \a] :cs [\d \b] :ds [\c \d] :corridor {}})

(def part2 {:as [\a \a \a] :bs [\b \b \b \b] :cs [\c \c \c \c] :ds [\d \d] :corridor {3 \d 9 \a 10 \d}})

(def input2 {:as [\b \d \d \c] :bs [\a \b \c \a] :cs [\d \a \b \b] :ds [\c \c \a \d] :corridor {}})

;(bfs input2)



(comment
  (-> example-state

      :state
      (move-amphipod :bs 5 :to-corridor)
      :state
      (move-amphipod :cs 5 :to-room)
      :state
      (move-amphipod :bs 5 :to-corridor)
      :state
      (move-amphipod :bs 3 :to-room)
      :state
      (move-amphipod :as 3 :to-corridor)
      :state
      (move-amphipod :bs 3 :to-room)
      :state
      (move-amphipod :ds 7 :to-corridor)
      :state
      (move-amphipod :ds 9 :to-corridor)
      :state
      (move-amphipod :ds 7 :to-room)
      :state
      (move-amphipod :ds 5 :to-room)
      :state
      (move-amphipod :as 9 :to-room)
      ))
