
// you can also use imports, for example:
// import java.util.*;

public class RakutenInterview {

    public static void main(String [] args) {
        // you can write to stdout for debugging purposes, e.g.
        System.out.println("This is a debug message");
        
        int[] arr1 = {3,1,5,0,7};
        int[] arr2 = {10, 0, -1, 10};
        RakutenInterview sol = new RakutenInterview();
        sol.sortArrays(arr1, arr2);
    }
    
    public void sortArrays(int[] array1, int[] array2) {
        int[] mergedArray = new int[array1.length + array2.length];
        int i = 0;
        
        // add all array1 to merged array
        for(i = 0; i < array1.length; i++) {
            mergedArray[i] = array1[i];
        }
        
        // append all array2 et the end of merged array
        for (int j = 0; j < array2.length; j++) {
            mergedArray[i++] =  array2[j];
        }
        
        int[] sorted = sort(mergedArray, 0, mergedArray.length-1);
        System.out.println("sorting: ");
        printArr(sorted);
    }
    
    public int[] sort(int[] arr, int start, int end) {
        int mid = (start + end) / 2;
        
        if (start == end) {
            int[] ret = new int[1];
            ret[0] = arr[start];
            return ret;
        }
        
        int[] left = sort(arr, start, mid);
        // printArr(left);
        int[] right = sort(arr, mid+1, end);
        int[] mergeArr = merge(left, right);
        return mergeArr;
    }
    
    public int[] merge(int[] left, int[] right) {
        // left and right are sorted
        int[] arr = new int[left.length + right.length];
        int i = 0;
        int j = 0;
        int k = 0; // index in merged arr
        
        while (i < left.length && j < right.length) {
            if (left[i] <= right[j]) {
                arr[k] = left[i];
                i++;
            } else {
                arr[k] = right[j];
                j++;
            }
            k++;
        }
        
        if (i < left.length && j >= right.length) {
            while (i < left.length) {
                arr[k++] = left[i++];
            }
        } else if (j < right.length && i >= left.length) {
            while (j < right.length) {
                arr[k++] = right[j++];
            }
        }
        
        return arr;
    }
    
    public void printArr(int[] arr) {
        for(int i = 0; i < arr.length; i++) {
            System.out.print(arr[i] + ", ");
        }
        System.out.println();
    }
}




/*
comment
3,1,5,0,7 - (1,3,5) - (0,7) -> 0,1,3,5,7
3,1,5, ,,, 5,5 -> 1,3, 5
3,1 -> 1,3
3,3 -> 1,1

arr, 0, 4
arr, 0, 2 | arr, 3, 4
arr, 0, 1 | arr 2, 2 , 

*/