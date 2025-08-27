echo "test content" > source.txt

cargo r cp source.txt destination.txt
cat destination.txt
//test content

cargo r cp one.txt destination.txt
//one

cargo r cp source.txt yy/
cat yy/source.txt
//test content

cargo r cp source.txt yy/destination.txt
cat yy/destination.txt
//test content

cargo r cp source.txt yy/nested/destination.txt
cat yy/nested/destination.txt
//test content




cargo r cp 
//err
//cp: missing file operand 
cargo r cp one.txt 
//err
//cp: missing destination file operand after 'one.txt'
cargo r cp one.txt destination.txt destination.txt
//err (3 ta3 files error but ila kano files o akhir argiment folder macho err 5ashom kamlin y copay fi dk folder) anther test
//cp: target 'dd': No such file or directory
cargo r cp notexist.txt destination.txt
//err
//cp: cannot stat 'jjkfdkjf': No such file or directory   





cargo r cp yy destination.txt
//err

cargo r cp source.txt dirnot/neww.txt
//err

cargo r cp source.txt yy/notexist/destination.txt
//err

cargo r cp source.txt notexistdir/
//err

cargo r cp source.txt yy/destination.txt/newww.txt
//err

cargo r cp source.txt notexistdir/destination.txt
//err

cargo r cp source.txt /notpermetion/newwww.txt
//err

cargo r cp unreadable.txt des.txt
//err

cargo r cp source.txt source.txt
//err

cargo r cp source.txt destination.txt/new.txt
//err