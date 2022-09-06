# Visualize.py by CoccaGuo at 2022/09/06 15:49

import numpy as np
import matplotlib.pyplot as plt

def sphere_to_rect(theta, phi):
    x = np.sin(theta) * np.cos(phi)
    y = np.sin(theta) * np.sin(phi)
    z = np.cos(theta)
    return x, y, z

if __name__ == '__main__':
    data = np.loadtxt('points.txt', dtype=np.float32)
    point_count = int(data.shape[1] / 2)
    step_count = data.shape[0]
    for i in range(step_count):
        fig = plt.figure("sphere points " + str(i))
        ax = fig.add_subplot(111, projection='3d')

        u = np.linspace(0, 2 * np.pi, 50)
        v = np.linspace(0, np.pi, 50)

        x = np.outer(np.cos(u), np.sin(v))
        y = np.outer(np.sin(u), np.sin(v))
        z = np.outer(np.ones(np.size(u)), np.cos(v))
        ax.plot_surface(x, y, z,  rstride=4, cstride=4, color='gray', linewidth=0, alpha=0.5)

        point_list = []
        for j in range(point_count):
            theta = data[i, j * 2]
            phi = data[i, j * 2 + 1]
            x, y, z = sphere_to_rect(theta, phi)
            point_list.append([x, y, z])
            ax.scatter(x, y, z, color='r', s=100)
        # connect theses points in point_list with red line
        for j in range(point_count):
            for k in range(j + 1, point_count):
                ax.plot([point_list[j][0], point_list[k][0]],
                        [point_list[j][1], point_list[k][1]],
                        [point_list[j][2], point_list[k][2]], color='r')
        # make the 3D plot seems like a sphere
        ax.set_xlim(-1.25, 1.25)
        ax.set_ylim(-1.25, 1.25)
        ax.set_zlim(-1, 1)
        plt.show()

