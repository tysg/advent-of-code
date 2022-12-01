(ns advent-of-code.day-16
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.data.priority-map :refer [priority-map]]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]))


(def example "EE00D40C823060")

(def hex->bits {
               "0"  '(0 0 0 0)
               "1"  '(0 0 0 1)
               "2"  '(0 0 1 0)
               "3"  '(0 0 1 1)
               "4"  '(0 1 0 0)
               "5"  '(0 1 0 1)
               "6"  '(0 1 1 0)
               "7"  '(0 1 1 1)
               "8"  '(1 0 0 0)
               "9"  '(1 0 0 1)
               "A"  '(1 0 1 0)
               "B"  '(1 0 1 1)
               "C"  '(1 1 0 0)
               "D"  '(1 1 0 1)
               "E"  '(1 1 1 0)
               "F"  '(1 1 1 1)
})


;; packet : {:ver :type :value}
;; {:ver :type}
;; top level packet: ver, type, subpackets 

;; 4 -> literal : {}
;; Packet |
;; Literal (int)
;; Opertar (List<Pakcets>)

(defn bits-to-int [bits] (Long/parseLong (string/join bits) 2))

(defn parse-hex [input] (->> input (re-seq #"\w") (map hex->bits) flatten))

(defn next-val [k bits] (let [[v r] (split-at k bits)] [(bits-to-int v) r]))

(defn take-while+
  [pred coll]
  (lazy-seq
    (when-let [[f & r] (seq coll)]
      (if (pred f)
        (cons f (take-while+ pred r))
        [f]))))


(defn literal-val [bits] 
  (let [groups (->> bits 
                    (partition 5) 
                    (take-while+ #(pos? (first %))))]
    [(->> groups (map rest) flatten bits-to-int)
     (drop (* 5 (count groups)) bits)]))

(defn packet [bits]
  (let [[pack-ver r] (next-val 3 bits)
        [pack-type r] (next-val 3 r)]
    (case pack-type
      4 (let [[val r] (literal-val r)] [{:ver pack-ver :type pack-type :val val} r])
      ;; else
      (let [[children r] (operator-children r)] [{:ver pack-ver :type pack-type :children children} r]))))

(declare packet operator-children)

(defn operator-children [bits]
  (let [[length-type r] (next-val 1 bits)]
    (case length-type
      0 (let [[l r] (next-val 15 r)] 
          [(loop [bits (take l r) res []]
             (if (empty? bits) res 
                 (let [[p r] (packet bits)]
                   (recur r (conj res p)))))
           (drop l r)])
      1 (let [[l r] (next-val 11 r)] 
          (loop [bits r 
                 res []]
            (if (= l (count res)) [res bits]
                (let [[p r] (packet bits)]
                  (recur r (conj res p)))))))))




(defn part-1 [input]
  (let [[pkt _] (packet (parse-hex input))]
    (reduce +  (map :ver (tree-seq :children :children pkt)))))


(defn eval [pkt]
  (if (= (:type pkt) 4)
    (:val pkt)
    (let [vals (map eval (:children pkt))]
      (case (:type pkt)
        0 (reduce + vals)
        1 (reduce * vals)
        2 (apply min vals)
        3 (apply max vals) 
        5 (if (apply > vals) 1 0)
        6 (if (apply < vals) 1 0)
        7 (if (apply = vals) 1 0)))))

(defn part-2 [input] (eval (first (packet (parse-hex input)))))

(part-2 (slurp "resources/day16.in"))
