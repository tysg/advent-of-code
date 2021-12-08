(ns advent-of-code.day-8
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]))


(def example 
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")

(def input (slurp "resources/day8.in"))

(defn parse-input [input]
  (map  #(re-seq #"\w+" %) (string/split-lines (string/trim input))))


(defn build-word-map [words]
  (map #(vector (count %) %) words))


(def mapping 
  "mapping from dash count to number"
  {3 7, 4 4, 2 1, 7 8})


(let [entries (parse-input input)
      rhs-word-maps (map #(build-word-map (second %)) entries)]
  
  (reduce + 
          (for [m rhs-word-maps                   
                :let [freqs (map first m)]]
            (count (filter mapping freqs))))
  ) 


(def segments [\a \b \c \d \e \f \g])

(def segment-perm (combo/permutations segments))

(defn build-char-map [permutation] (zipmap segments permutation))

(def segment-mapping 
{
 #{\a \b \c \e \f \g} \0
 #{\c \f} \1
 #{\a \c \d \e \g} \2
 #{\a \c \d \f \g} \3
 #{\b \c \d \f} \4
 
 #{\a \b \d \f \g} \5
 #{\a \b \d \e \f \g} \6
 #{\a \c \f} \7
 #{\a \b \c \d \e \f \g} \8
 #{\a \b \c \d \f \g} \9
 })

(defn replace-with-mapping [words mapping]
  (for [w words] (replace mapping w)))

(def all-mappings (map build-char-map segment-perm))

(defn all-valid-digits? [words] 
  (every? segment-mapping (map #(into #{} %) words)))


(defn find-valid-mappings [words]
  (filter
   (fn [mapping] (all-valid-digits? (replace-with-mapping words mapping)))
   all-mappings))


(let [lines (parse-input input)]
  (reduce +  (map
              (fn [words] 
                (let [mapping (first (find-valid-mappings words))]
                  (->> (replace-with-mapping words mapping)
                       (map (fn [s] (into #{} s)))
                       (map segment-mapping)
                       (take-last 4)
                       (string/join)
                       (Long/parseLong)
                       )))
              lines))
)
   



