(ns advent-of-code.day-18
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]
   [clojure.zip :as z]))

(def small "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]")

(def larger "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]")


(defn next-leaf [loc]
  (loop [loc (z/next loc)]
    (cond 
      (z/end? loc) nil
      (int? (z/node loc)) loc
      :else (recur (z/next loc)))))

(defn prev-leaf [loc]
  (loop [loc (z/prev loc)]
    (cond 
      (nil? loc) nil
      (int? (z/node loc)) loc
      :else (recur (z/prev loc)))))

(defn tree-walk [root]
  (->> root
       (iterate z/next)
       (take-while (complement z/end?))))

(defn to-explode [loc]
  (->> (tree-walk loc)
       (filter #(let [n (z/node %)] 
                  (and (vector? n) (int? (first n)) (int? (second n)))
                  ))
       (filter #(>= (count (z/path %)) 4))
       first))

(defn to-split [loc]
  (->> (tree-walk loc)
       (filter #(let [n (z/node %)] (and (int? n) (>= n 10))))
       first))

(defn explode [loc]
  (let [[l r] (z/node loc)
        loc' (z/replace loc 0)
        loc' (if-let [prev (prev-leaf loc')]
               (next-leaf (z/edit prev + l))
               loc')
        loc' (if-let [next (next-leaf loc')]
               (z/edit next + r)
               loc')]

    (z/vector-zip (z/root loc'))))

(defn split [loc]
  (let [half (/ (z/node loc) 2)
        n (mapv int [(Math/floor half) (Math/ceil half)])]
    (-> loc (z/replace n) z/root z/vector-zip)))

(defn reduction [loc]
  (if-let [exploder (to-explode loc)] 
    (recur (explode exploder))
    (if-let [splitter (to-split loc)]
      (recur (split splitter))
      (z/node loc))))

(defn magnitude [root]
  (if (int? root) 
    root
    (let [[l r] root]
      (+ (* 3 (magnitude l))
         (* 2 (magnitude r))))))

(defn parse-input [input] (map read-string (string/split-lines input)))

(defn part-1 [input] 
  (->> (parse-input input)
       (reduce (fn [a b] (reduction (z/vector-zip [a b]))))
       magnitude))

(defn part-2 [input]
  (let [l (parse-input input)]
    (apply max
           (for [a l b l]
             (magnitude (reduction (z/vector-zip [a b])))))))

;(part-2 (slurp "resources/day18.in"))



