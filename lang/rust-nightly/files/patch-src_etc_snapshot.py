--- src/etc/snapshot.py.orig	2015-08-30 16:17:32.974828000 +0200
+++ src/etc/snapshot.py	2015-08-30 16:38:21.974971000 +0200
@@ -132,6 +132,11 @@
                     "log", "-n", "1",
                     "--format=%%%s" % field, "HEAD"])
 
+def local_rev_info(field):
+    lut = {'H': 2, 'h': 1, 'ci': 0}
+    with open(os.path.join(src_dir, 'git-log-info')) as f:
+        info = f.readline().strip().split(':')
+    return info[lut[field]]
 
 def local_rev_full_sha():
     return local_rev_info("H").split()[0]
