<?php
$options = array(
'path' => '',
'secure' => true,
'httponly' => true,
'samesite' => 'Strict'
);
// $options['samesite']='Lax';
session_set_cookie_params($options);
session_start();
session_write_close();
?>

<!doctype html>
<html>
<head>
<title>samesite test</title>
</head>
<body>
  *  Open Debugger window and observe session cookie is set.</br>
  *  Press browser refresh. Observe that no cookie was sent to the server. After refresh there is a new cookie.</br>
  *  Transmit the URL from the address bar. Observe that a cookie was sent to the server and we keep the same cookie.</br>
  *  Press browser refresh agai. Observe that a cookie was sent to the server and we keep the same cookie.</br>
</body>
</html>
