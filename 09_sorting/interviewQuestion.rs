// NOTE: Translated original answers into English

/*
  #1 - Sort 10 schools around your house by distance:
        Use Insertion Sort because the input size is small, and in these cases, this algorithm is
        very effective. Moreover, if the input is (or almost) sorted, it will be even faster.

  #2 - eBay sorts listings by the current Bid amount:
        Use Radix or Counting Sort because it involves numbers within a range, which allows for
        fast executions with numbers. It's also an integer value, and since it's eBay,
        the values won't be as large as a trillion, they will be smaller values.

  #3 - Sport scores on ESPN:
        Use Quick Sort because sometimes scores have decimals and different formats for football,
        tennis, etc. It will be the most efficient because despite worrying about the worst case
        since entries can be different, it's important to consider the memory space we will use.

  #4 - Massive database (can't fit all into memory) needs to sort through past year's user data:
        Use Merge Sort because we won't be sorting in memory since the data is very large, and
        because the size is massive we must prioritize time complexity.

  #5 - Almost sorted Udemy review data needs to update and add 2 new reviews:
        Use Insertion Sort because the data is almost sorted, and we are only adding 2 reviews.

  #6 - Temperature Records for the past 50 years in Canada:
        Use Radix or Counting Sort because there are no decimals, and they are usually two-digit
        numbers, which allows us to take advantage of the resources by using these algorithms.

  #7 - Large user name database needs to be sorted. Data is very random:
        Use Merge Sort if we have enough memory, or Quick Sort if we are not concerned about the
        worst case and can choose a good pivot.

  #8 - You want to teach sorting for the first time:
        Use Bubble Sort, Selection Sort because they are the simplest.

  ANSWERS:
  #1 - Sort 10 schools around your house by distance:
  insertion sort

  #2 - eBay sorts listings by the current Bid amount:
  radix or counting sort

  #3 - Sport scores on ESPN:
  quick sort

  #4 - Massive database (can't fit all into memory) needs to sort through past year's user data:
  merge sort

  #5 - Almost sorted Udemy review data needs to update and add 2 new reviews:
  insertion sort

  #6 - Temperature Records for the past 50 years in Canada:
  radix or counting sort
  quick sort if decimal places

  #7 - Large user name database needs to be sorted. Data is very random:
  quick sort

  #8 - You want to teach sorting for the first time:
  bubble sort

*/
