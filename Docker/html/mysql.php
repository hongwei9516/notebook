<?php
$servername = "mysql";
$username = "root";
$password = "allen";

 
// 创建连接
$conn = mysqli_connect($servername, $username, $password);
 
// 检测连接
if (!$conn) {
    die("Connection failed: " . mysqli_connect_error());
}
echo "connect success...";
?>
