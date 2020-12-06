compare_all <- FALSE
if (compare_all) {
  sizes <- scan("sizes.txt", what = integer(), sep = " ")
  selection_totals <- scan("selection_totals.txt", what = integer(), sep = " ")
  insertion_totals <- scan("insertion_totals.txt", what = integer(), sep = " ")
  bubble_totals <- scan("bubble_totals.txt", what = integer(), sep = " ")
  merge_totals <- scan("merge_totals.txt", what = integer(), sep = " ")
  heap1_totals <- scan("heap1_totals.txt", what = integer(), sep = " ")
  heap2_totals <- scan("heap2_totals.txt", what = integer(), sep = " ")
  name <- "Blue: selection, Yellow: insertion, Chocolate: bubble, DarkGreen: merge, Red: heap v1, VioletRed: heap v2"
  options(scipen=5)
  x.min.value <- min(sizes)
  x.max.value <- max(sizes)
  y.min.value <- min(c(selection_totals, insertion_totals, bubble_totals, merge_totals, heap1_totals, heap2_totals))
  y.max.value <- max(c(selection_totals, insertion_totals, bubble_totals, merge_totals, heap1_totals, heap2_totals))
  plot(sizes, selection_totals, type = "l", col = "blue", main = name, xlab = "Array size", ylab = "Time", ylim = c(y.min.value, y.max.value), xlim = c(x.min.value, x.max.value))
  lines(sizes, insertion_totals, type = "l", col = "yellow")
  lines(sizes, bubble_totals, type = "l", col = "chocolate")
  lines(sizes, merge_totals, type = "l", col = "darkgreen")
  lines(sizes, heap1_totals, type = "l", col = "red")
  lines(sizes, heap2_totals, type = "l", col = "violetred")
} else { #Compare only heap sorts and merge sort
  sizes <- scan("sizes.txt", what = integer(), sep = " ")
  merge_totals <- scan("merge_totals.txt", what = integer(), sep = " ")
  heap1_totals <- scan("heap1_totals.txt", what = integer(), sep = " ")
  heap2_totals <- scan("heap2_totals.txt", what = integer(), sep = " ")
  name <- "DarkGreen: merge, Red: heap v1, VioletRed: heap v2"
  options(scipen=5)
  x.min.value <- min(sizes)
  x.max.value <- max(sizes)
  y.min.value <- min(c(merge_totals, heap1_totals, heap2_totals))
  y.max.value <- max(c(merge_totals, heap1_totals, heap2_totals))
  plot(sizes, heap1_totals, type = "l", col = "red", main = name, xlab = "Array size", ylab = "Time", ylim = c(y.min.value, y.max.value), xlim = c(x.min.value, x.max.value))
  lines(sizes, heap2_totals, type = "l", col = "violetred")
  lines(sizes, merge_totals, type = "l", col = "darkgreen")
}


