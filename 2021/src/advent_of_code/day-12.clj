(ns advent-of-code.day-12
  (:require
   [clojure.string :as string]
   [clojure.set :as set]
   [clojure.math.combinatorics :as combo]
   [clojure.pprint :as pp]
   [clojure.test :refer [is run-tests]]))

(def small "start-A
start-b
A-c
A-b
b-d
A-end
b-end
")

(def large "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW")

(def input "ey-dv
AL-ms
ey-lx
zw-YT
hm-zw
start-YT
start-ms
dv-YT
hm-ms
end-ey
AL-ey
end-hm
rh-hm
dv-ms
AL-dv
ey-SP
hm-lx
dv-start
end-lx
zw-AL
hm-AL
lx-zw
ey-zw
zw-dv
YT-ms")

(defn parse-graph [input]
  (reduce
   (fn [g line] (let [[_ start end] (re-matches #"(\w+)-(\w+)" line)]
                  (-> g
                      (update start #(if (nil? %) [end] (conj % end)))
                      (update end #(if (nil? %) [start] (conj % start))))))
   {}
   (string/split-lines input)))


(defn visit [graph root visited prevs]
  (if (= root "end")
    (vector (conj prevs "end"))
    (let [to-visit (filter #(nil? (visited %)) (graph root))]
      (mapcat (fn [v] (visit 
                       graph
                       v
                       (if (every? #(Character/isLowerCase %) root) (conj visited root) visited)  
                       (conj prevs root)))
              to-visit))))

(defn visit-2 [graph visited prevs allow-small]
  (let [curr (peek prevs)]
    (if (= "end" curr)
      (vector prevs)
      (->> curr
           graph
           (remove #(= "start" %))
           (remove (fn [n] (and (visited n) (every? #(Character/isLowerCase %) n) (not allow-small))))
           (mapcat (fn [n] (visit-2 graph (conj visited n) (conj prevs n) (if (and  (visited n) allow-small (every? #(Character/isLowerCase %) n)) false allow-small)))))))

)

(let [graph  (parse-graph input)]
  (count (visit graph "start" #{} [])))

(let [graph  (parse-graph medium)]
  (count  (set (visit-2 graph #{"start"} ["start"] true))))



(def medium "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc")
