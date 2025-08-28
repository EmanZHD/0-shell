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
cargo r cp file1 file2 newfile
//err file lakhar makayanch 
//cp: target 'newfile' is not a directory
cargo r cp file1 file2 existing_file
//err akhir wa7ad file machi dir
//cp: target 'existing_file' is not a directory
cargo cp file file folder file file dolder
//err source 5as ykono files kamlin















cargo r cp *.txt folder/
// copy all files with .txt in filder 
















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


//t9ad tala3 ktar mn err 
// like:
// cp kk myd mydir tt rust  
//    cp: -r not specified; omitting directory 'mydir'    
//       cp: cannot stat 'tt': No such file or directory 