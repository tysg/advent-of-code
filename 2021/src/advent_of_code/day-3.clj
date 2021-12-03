(ns advent-of-code.day-3
  (:require
   [clojure.string :as string]
;   [clojure.test]
))


(defn get-common-bit [mode s]
  (let [ones (count (filter #(= \1 %) s))
        zeros (- (count s) ones)]
    (case mode
      :most (if (<= zeros ones) \1 \0)
      :least (if (<= zeros ones) \0 \1))))

;; (deftest test-get-common-bit
;;   (is (= \1 
;;          (get-common-bit :most '(\0 \1 \1 \1 \1 \0 \0)))))

(defn zip-seq [num-seq]
  "returns a sequence of all chars at index 0, 1, 2, .."
   (->> num-seq
       (apply interleave)
       (partition (count num-seq))))

(defn bin-seq->int [bin-chars]
  (Integer/parseInt (string/join bin-chars) 2))


(defn find-rate [mode bits-by-index]
  (bin-seq->int (map #(get-common-bit mode %) bits-by-index)))


(defn get-diagnostics [s]
  (let [bits-by-index (zip-seq s)]
    [(find-rate :most bits-by-index) 
     (find-rate :least bits-by-index)]))


(defn calc-rating [input-binary-seq curr-idx filter-mode]
    (if (= (count input-binary-seq) 1)
      (bin-seq->int (first input-binary-seq))
      (let [bits-by-position (zip-seq input-binary-seq)
            common-bit  (get-common-bit filter-mode (nth bits-by-position curr-idx))]
        (recur (filter #(= common-bit (nth % curr-idx)) input-binary-seq) 
               (inc curr-idx) 
               filter-mode ))))

(let [input-str (slurp "resources/day3a.in")
      lines (string/split input-str #"\n" )
      char-vecs (map vec lines)]
  ;(apply * (get-diagnostics lines)) ; part 1
  (* (calc-rating char-vecs 0 :most) (calc-rating char-vecs 0 :least)) ; part 2
)

