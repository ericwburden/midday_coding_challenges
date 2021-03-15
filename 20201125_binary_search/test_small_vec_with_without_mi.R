source('binary_search.R')


#' Testing parameters
part_range <- seq(.05, .95, .05)
lengths <- c(20, 320, 5120, 81920, 1310720)
reps <- 10000

# Run the battery of tests
final_tally <- map(lengths, ~ test_length(.x, reps, part_range)) %>% 
  bind_rows()

#' Normalizing the results helps to visualize them on a chart. I ended up 
#' removing the top and bottom of the partition range since those values were
#' bad enough they made it difficult to tell the differences between performance
#' for partitions in the middle of the range on the chart.
normalized_tally <- final_tally %>% 
  group_by(length) %>% 
  mutate(norm_rounds = mean_rounds/max(mean_rounds)) %>% 
  ungroup()

#' Visualization!
ggplot(normalized_tally, aes(part, norm_rounds, color = found_mi)) +
  geom_point() +
  geom_line() +
  facet_wrap(~ length, nrow = 1, labeller = label_both) +
  theme_minimal() +
  scale_color_manual(values = c('goldenrod', 'navyblue')) +
  theme(
    legend.position = c(.925, .85),
    legend.background = element_rect(fill = 'white', size = 0.5),
    plot.caption = element_text(color = "gray25", face = "italic", margin = margin(t = 10))
  ) +
  labs(
    x = 'Partition Value',
    y = 'Normalized # of Rounds',
    color = "Found the\nMagic Index",
    title = 'Effect of Partition and Vector Length on Binary Search',
    subtitle = 'Higher partition values yield faster searches at lower vector lengths',
    caption = 'Each combination of vector length and partition value was tested 10,000 times to obtain these results'
  )
