(ns advent-of-code.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))

(defn parse-int-series
  [input]
  (map #(Integer/parseInt %)
       (str/split input #"\n")))


