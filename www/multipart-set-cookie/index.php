<?php
header("Content-Type: multipart/x-mixed-replace; boundary=TEST");
?>
--TEST
Set-Cookie: testcookie=testvalue

<h1>Test</h1>
--TEST--
