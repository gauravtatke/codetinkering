public class MergeSort {
    public static void main(String[] args) {
        int[] arr = {10, 0, -1, 10};
        MergeSort.sort(arr, 0, arr.length-1);
        System.out.println("Sorted array: ");
        printArr(arr);
    }

    public static void sort(int[] arr, int start, int end) {
        if (start < end) {
            int mid = (start + end)/2;
            sort(arr, start, mid);
            sort(arr, mid+1, end);
            merge(arr, start, mid, end);
        }
    }

    public static void merge(int[] arr, int start, int mid, int end) {
        int n1 = mid - start + 1;
        int n2 = end - mid;
        int[] left = new int[n1];
        int[] right = new int[n2];
        
        int k = 0;
        for (int i = start; i <= mid; i++) {
            left[k++] = arr[i];
        }

        k = 0;
        for(int i = mid+1; i <= end; i++) {
            right[k++] = arr[i];
        }
        
        int l = 0;
        int r = 0;
        k = start;
        while (l < n1 && r < n2) {
            if (left[l] <= right[r]) {
                arr[k++] = left[l++];
            } else {
                arr[k++] = right[r++];
            }
        }

        if (l < n1) {
            while (l < n1) {
                arr[k++] = left[l++];
            }
        } else {
            while (r < n2) {
                arr[k++] =  right[r++];
            }
        }
    }

    public static void printArr(int[] arr) {
        for(int i: arr) {
            System.out.print(i + ", ");
        }
        System.out.println();
    }

}