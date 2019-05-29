# Copyright (c) 2019 Karl Sundequist Blomdahl <karl.sundequist.blomdahl@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

from .conv2d import conv2d
from .global_avg_pool import global_avg_pool
from .named import named
from .softmax import softmax


def global_avg_pooling_classifier(x, num_outputs, name=None):
    """ Returns a head that outputs `num_outputs` classes, using a _Global
    Average Pooling_ architecture [1].

    [1] https://arxiv.org/pdf/1312.4400.pdf, _Network In Network_, Section 3.2
    """
    conv_x = conv2d(x, num_outputs, [1, 1], activation='linear')
    gap_x = global_avg_pool(conv_x)
    softmax_x = softmax(gap_x)

    y = named(softmax_x, name=name)

    return y