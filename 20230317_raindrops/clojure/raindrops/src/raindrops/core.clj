(ns raindrops.core
  (:require [clojure.string :as str]))

(def rain-factors '([3 "Pling"] [5 "Plang"] [7 "Plong"]))

(defn convert
  "I don't do a whole lot."
  [number]
  (loop [[first & rest] rain-factors
         number number
         rain-sounds ""]
    (let [[factor sound] first]
      (if-not factor (if (str/blank? rain-sounds) (str number) rain-sounds)
        (if (= 0 (mod number factor)) 
          (recur rest number (str rain-sounds sound))
          (recur rest number rain-sounds))))))
