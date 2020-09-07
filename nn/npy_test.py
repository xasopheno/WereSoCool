import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
import csv
from torchvision import transforms
from torch.utils.data import DataLoader, Dataset
from network import Generator, Discriminator, weights_init
from typing import List
import random
import os


def normalize_data_to_tanh_space(x: np.array) -> np.array:
    return x * 2.0 - 1.0


class RealDataGenerator(Dataset):
    def __init__(self, files: List[str]):
        self.x = np.array([])
        self.files = files

    def __len__(self):
        return len(files)

    def prepare_image(self, x: np.array) -> np.array:
        x = x[:64]
        x = normalize_data_to_tanh_space(x)
        padding = np.array(
            [np.zeros_like(x[0]) - 1.0 for i in range(x[0].shape[0] - x.shape[0])]
        )
        x = np.concatenate([x, padding])

        #  print("data_shape:", x.shape)
        x = np.array([x[:, :, i] for i in range(7)])
        #  print("transformed_shape:", im.shape)
        #  print(im)
        return x

    def __getitem__(self, idx: int):
        n_steps = None
        op_len = None
        with open(self.files[idx]) as csv_file:
            x = np.array([])
            csv_reader = csv.reader(csv_file, delimiter=",")
            line_count = 0
            for row in csv_reader:
                if line_count == 0:
                    r = list(map(int, row))
                    n_steps, op_len = r

                    line_count += 1
                    continue

                r = np.array(list(map(float, row)))
                x = np.append(x, [r])
                line_count += 1
            #  print("lines in file:", line_count)

        #  print("n_steps:", n_steps)
        #  print("op_len:", op_len)
        x = x.reshape(-1, n_steps, op_len)

        return torch.tensor(self.prepare_image(x))


files = []
dirs = ["data/how_to_rest"]
for d in dirs:
    for f in os.listdir(d):
        if f.endswith(".csv"):
            #  print(os.path.join(d, f))
            files.append(os.path.join(d, f))

#  print(files)

files = files[0:1000]
r = RealDataGenerator(files)
if __name__ == "__main__":
    nz = 256
    batch_size = 8
    device = "cuda"
    fixed_noise = torch.randn(batch_size, nz, 1, 1, device=device)
    ngpu = 2
    real_label = 1
    fake_label = 0
    epochs = 100
    lr = 0.0001
    beta1 = 0.5
    criterion = nn.BCELoss()

    netG = Generator(ngpu).to(device, dtype=torch.float)
    netG.apply(weights_init)
    #  if opt.netG != "":
    #  netG.load_state_dict(torch.load(opt.netG))
    print(netG)

    netD = Discriminator(ngpu).to(device, dtype=torch.float)
    netD.apply(weights_init)
    print(netD)
    #  if opt.netD != "":
    #  netD.load_state_dict(torch.load(opt.netD))

    optimizerD = optim.Adam(netD.parameters(), lr=lr, betas=(beta1, 0.999))
    optimizerG = optim.Adam(netG.parameters(), lr=lr, betas=(beta1, 0.999))

    for epoch in range(epochs):
        for i in range(len(r)):
            ############################
            # (1) Update D network: maximize log(D(x)) + log(1 - D(G(z)))
            ###########################
            # train with real
            #  print("real________")
            netD.zero_grad()
            real_batch = [
                r[random.randint(0, len(r) - 1)].numpy() for i in range(batch_size)
            ]

            real_batch = torch.tensor(real_batch).to(device, dtype=torch.float)
            label = torch.full(
                (batch_size,), real_label, dtype=torch.float, device=device
            )

            output = netD(real_batch)
            errD_real = criterion(output, label)
            errD_real.backward()
            D_x = output.mean().item()

            noise = torch.randn(batch_size, nz, 1, 1, device=device, dtype=torch.float)
            fake = netG(noise)
            label.fill_(fake_label)
            output = netD(fake.detach())
            errD_fake = criterion(output, label)
            errD_fake.backward()
            D_G_z1 = output.mean().item()
            errD = errD_real + errD_fake
            optimizerD.step()

            ############################
            # (2) Update G network: maximize log(D(G(z)))
            ###########################
            netG.zero_grad()
            label.fill_(real_label)  # fake labels are real for generator cost
            output = netD(fake)
            errG = criterion(output, label)
            errG.backward()
            D_G_z2 = output.mean().item()
            optimizerG.step()

            if i % 10 == 0:
                print(
                    "[%d/%d][%d/%d] Loss_D: %.4f Loss_G: %.4f D(x): %.4f D(G(z)): %.4f / %.4f"
                    % (
                        epoch,
                        epochs,
                        i,
                        len(r),
                        errD.item(),
                        errG.item(),
                        D_x,
                        D_G_z1,
                        D_G_z2,
                    )
                )

        print(
            "[%d/%d][%d/%d] Loss_D: %.4f Loss_G: %.4f D(x): %.4f D(G(z)): %.8f / %.8f"
            % (epoch, epochs, i, len(r), errD.item(), errG.item(), D_x, D_G_z1, D_G_z2,)
        )
        #  if i % 100 == 0:
        #  vutils.save_image(
        #  real_cpu, "%s/real_samples.png" % opt.outf, normalize=True
        #  )
        #  fake = netG(fixed_noise)
        #  vutils.save_image(
        #  fake.detach(),
        #  "%s/fake_samples_epoch_%03d.png" % (opt.outf, epoch),
        #  normalize=True,
        #  )

        #  if opt.dry_run:
        #  break

    #  op_len = 7
    #  n_steps = 4
    #  n_voices = 2
    #  n_examples = 3

    #  x = np.arange(n_steps * op_len * n_voices * n_examples)

