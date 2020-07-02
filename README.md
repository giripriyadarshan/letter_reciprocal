# letter_reciprocal
Written in Rust-Lang.
Prints reciprocal letter of any string you type
(Translated from C in a proper manner)
This is the original code i wrote in C
```c
#include<stdio.h>
int main()
{
 int i,j;
 char a[26]="abcdefghijklmnopqrstuvwxyz";
 char b[50];
 printf("Enter the string\n");
 gets(b);
 // scanf("%s",b);//use scanf() if the compiler throws error with gets() function
 for(int i=0;b[i]!='\0';i++)
  for(j=0;a[j]!='\0';j++)
   if(b[i]==a[j])
    {
     b[i]=a[25-j];
     break;
    }
 printf("%s\n",b );
 return 0;
}
```
