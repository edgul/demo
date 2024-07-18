<?php
header('Content-Type: text/html');

// set 11 cookies of size 1000 Bytes -> 11000 Bytes
for ($i = 1; $i <= 110; $i++) {
  $name = "bigcookie$i";
  $needbytes = 100 - strlen($name); 
  $value = str_repeat('_', $needbytes);

  // can't set Partitioned attribute with setcookie(), use header()
  header("Set-Cookie: $name=$value; Path=/; Max-Age=3600; SameSite=None; Secure; Partitioned", false);
}

?>

<!-- Serve locally with `php -S localhost:8000` -->
<!DOCTYPE html>
Mostly empty content, but we set 11KB worth of cookies. <br>
