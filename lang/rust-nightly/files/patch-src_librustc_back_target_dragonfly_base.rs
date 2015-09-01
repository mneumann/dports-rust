--- src/librustc_back/target/dragonfly_base.rs.orig	2015-07-17 18:18:52.000000000 +0200
+++ src/librustc_back/target/dragonfly_base.rs	2015-09-01 23:00:57.937410000 +0200
@@ -29,7 +29,7 @@
             "-Wl,--as-needed".to_string(),
         ),
         position_independent_executables: true,
-        archive_format: "bsd".to_string(),
+        archive_format: "gnu".to_string(),
         .. Default::default()
     }
 }
