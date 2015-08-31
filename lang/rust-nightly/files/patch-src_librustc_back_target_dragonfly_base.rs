--- src/librustc_back/target/dragonfly_base.rs.orig	2015-07-26 12:39:18.000000000 +0200
+++ src/librustc_back/target/dragonfly_base.rs	2015-08-31 09:18:43.649340000 +0200
@@ -29,7 +29,7 @@
             "-Wl,--as-needed".to_string(),
         ),
         position_independent_executables: true,
-        archive_format: "bsd".to_string(),
+        archive_format: "gnu".to_string(),
         .. Default::default()
     }
 }
