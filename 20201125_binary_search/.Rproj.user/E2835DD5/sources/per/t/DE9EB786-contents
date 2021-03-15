#' Testing parameters
part_range <- seq(.05, .95, .1)
magnitudes <- c(1:6)
reps <- 500

# Run the battery of tests
final_tally <- map(magnitudes, ~ test_magnitude(.x, reps, part_range)) %>% 
  bind_rows()

#' Normalizing the results helps to visualize them on a chart. I ended up 
#' removing the top and bottom of the partition range since those values were
#' bad enough they made it difficult to tell the differences between performance
#' for partitions in the middle of the range on the chart.
normalized_tally <- final_tally %>% 
  group_by(magnitude) %>% 
  mutate(norm_rounds = mean_rounds/max(mean_rounds)) %>% 
  ungroup() %>% 
  filter(part > .3, part < .9)

#' Visualization!
ggplot(normalized_tally, aes(part, norm_rounds, color = factor(magnitude))) +
  geom_point() +
  geom_line() +
  facet_wrap(~ magnitude, ncol = 1)