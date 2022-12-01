(ns advent-of-code.day-14
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]
   [clojure.tools.trace :refer [deftrace]]))

(def example "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
")


(defn parse-input [input]
  (let [[base-line rule-lines] (string/split input #"\n\n")
        base (frequencies (partition 2 1 base-line))
        
        rules (into 
               {} 
               (map (fn [l] (let [[_ p1 p2 t] (re-matches #"([A-Z])([A-Z]) -> ([A-Z])" l)]
                              [(vector (first p1) (first p2)) (first t)])) 
                    (string/split-lines (string/trim rule-lines))))]
    [base rules]))


(defn char-freq [pair-freq]
  (let [freq-last (reduce
                   (fn [d [[_ p2] freq]] (merge-with + d {p2 freq}))
                   {}
                   pair-freq)]
    (merge-with + freq-last {(first (ffirst pair-freq)) 1})))

(let [[base rules] (parse-input (slurp "resources/day14.in"))
      build (fn [base] (reduce 
                        (fn [acc [k freq]] 
                          (if-let [t (rules k)]
                            (merge-with + acc
                                        {[(first k) t] freq}
                                        {[t (second k)] freq})
                            (merge-with + acc
                                        {k freq})))
                        {}
                        base))
      freqs (-> (iterate build base) (nth 40) char-freq vals)]
  (- (apply max freqs) (apply min freqs)))
