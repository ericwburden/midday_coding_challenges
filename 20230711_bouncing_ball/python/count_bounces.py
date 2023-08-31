import math

def count_bounces(h, bounce, window):
    if h <= 0: return -1
    if not 0 < bounce < 1: return -1
    if window >= h: return -1

    result = 1
    while (h * bounce > window):
        result += 2
        h *= bounce

    return result

def count_bounces_with_math(h, bounce, window):
    if h <= 0: return -1
    if not 0 < bounce < 1: return -1
    if window >= h: return -1

    """
    The height of the ball, once dropped, decays logarithmically. This is
    easily demonstrated in terms of a ball dropped from a height of 100 ft
    where the ball bounces half as high each time, aka `bounce` = 0.5. The
    bounces go like this:

      - Dropped From: 100ft
      - First Bounce:  50ft
      - Second Bounce: 25ft
      - Third Bounce:  12.5ft
      - Fourth Bounce:  6.25ft
      - Fifth Bounce:   3.125ft
      - Sixth Bounce:   1.5625ft
      - Seventh Bounce: 0.78125ft
    
    We see that the last bounce to a window at 1ft high is the sixth. The last
    bounce tow a window at 10ft high is the third, etc. So, the formula for the
    number of bounces at a certain window height is...
      log_(1/bounce)(height / window) = # bounces to a window
    
    So, log2(100 / 10) = 3.322, or, a ball dropped from 100ft with 0.5 bounce
    will bounce to the height of a 10ft window 3.322 times.
    """
    bounces = math.log(h / window, 1 / bounce)

    # Of course, we only care about how many times the ball _actually_ reaches the
    # height of the window, not any _partial_ attempts.
    (bounces, remaining) = divmod(bounces, 1)

    # Of course, we have the constraint that all bounces must be _greater_ than
    # the height of the window, not just equal. This means that we need to check 
    # to see if the ball had any bounce left the last time it reached the height
    # of the window to know if it went _above_ the window and count that last bounce.
    if not remaining:
        bounces -= 1

    # The ball will pass the window two times for each bounce, up and down, 
    # plus one for the initial drop past window
    return (bounces * 2) + 1
