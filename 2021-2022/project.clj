(defproject advent-of-code "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [[org.clojure/clojure "1.10.3"]
                 [org.clojure/math.combinatorics "0.1.6"]
                 [org.clojure/tools.trace "0.7.11"]
                 [org.clojure/data.priority-map "1.1.0"]
                 [com.clojure-goes-fast/clj-async-profiler "0.5.1"]
                 [org.clojure/core.match "1.0.0"]
]



  :jvm-opts ["-Djdk.attach.allowAttachSelf"]
  :main ^:skip-aot advent-of-code.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
