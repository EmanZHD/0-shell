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
cargo r cp file *.txt folder/
// copy all files with .txt in filder 
cargo r cp file *.txt folder/
//one fo this .txt if not file
//err
//cp: -r not specified; omitting directory 'ttgff.txt' 




cargo r cp file *.txt file/
// distination machi folder 7it 3andi ktar mn file kaysali b .txt
//err
//cp: target 'kk': Not a directory   jj kk older

cargo r cp file "*.txt" dest/
//ila kan "" hada kay3ni anaho file smito *.txt ya3ni mayataba9ch 3lih lalgo ta3 * 

cargo r cp file *.txt folder/
//mkyn ta wa7ad kaysali bi .txt
//err
//cp: cannot stat '*.txt': No such file or directory 

cargo r cp one.txt * folder 

















cargo r cp myfile newfile
//valid




















cargo r cp file folder
//valid
cargo r cp folder folder
//err source folder noo
//cp: -r not specified; omitting directory 'my' 
cargo r cp file file 
// Copy a file onto itself
//cp: 'kk' and 'kk' are the same file   
cargo r cp file file2
// file2 fayt kayn overwritter
cargo r cp files file
// files makaynch err
//cp: cannot stat 'ddd': No such file or directory 
cargo r cp *.txt folder/
// copy all files with .txt in filder 

cargo r cp *.txt folder/
//one fo this .txt if not file
//err
//cp: -r not specified; omitting directory 'folderr.txt'  

cargo r cp *.txt folder/
//mkyn ta wa7ad kaysali bi .txt
//err
//cp: cannot stat '*.txt': No such file or directory 

cargo r cp *.txt file/
//source ri file wahad  ya3ni 3andi ri file wa7ad kaysali b .txt machi darori distination ykon folder
//machi err
//valid


cargo r cp "lll" dest/
//ila kan "" hada kay3ni anaho file smito *.txt ya3ni mayataba9ch 3lih lalgo ta3 * 



cargo r cp .* destination.txt
//Copy with hidden files



cargo r cp source.txt dirnot/neww.txt
//err
//cp: cannot stat 'dirnot': No such file or directory 

cargo r cp source.txt yy/notexist/destination.txt
//err
//cp: cannot stat 'notexist': No such file or directory 

cargo r cp source.txt notexistdir/
//err
//cp: cannot stat 'notexist': No such file or directory

cargo r cp source.txt notexistdir/destination.txt
//err


cargo r cp unreadable.txt des.txt
//err Copy a file you don’t have read permission on
//cp: cannot open 'kk' for reading: Permission denied 

cargo r cp file.txt unwriter.txt
//err Copy to a directory without write permission
//cp: cannot create regular file 'kk': Permission denied 


//(hado joj zidihom ta fi multible)



//t9ad tala3 ktar mn err 
// like:
// cp kk myd mydir tt rust  
//    cp: -r not specified; omitting directory 'mydir'    
//       cp: cannot stat 'tt': No such file or directory 