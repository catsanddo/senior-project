import pygame as pg
import json
pg.init()

black = (0, 0, 0)
white = (255, 255, 255)


class Wall:
    def __init__(self, x, y):
        self.x = x
        self.y = y


def draw_text(text, x, y):
    screen = pg.display.get_surface()
    font = pg.font.SysFont('GNU', 50)
    screen.blit(font.render(text, True, white), (x, y))


def save_scene(walls, player):
    print('Saving...')

    print('Input file name: ')
    file_name = ''
    run = True
    while run:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                return False
            if e.type == pg.KEYDOWN:
                if e.key == pg.K_RETURN:
                    run = False
                elif e.key == pg.K_ESCAPE:
                    print('Canceled!\n')
                    return True
                elif e.key == pg.K_BACKSPACE:
                    file_name = file_name[:-1]
                else:
                    file_name += e.unicode
        pg.display.get_surface().fill(black, (0, 0, 512, 55))
        draw_text('Save: ' + file_name, 10, 10)
        pg.display.flip()
    print()
    if not len(file_name):
        file_name = 'level.json'

    scene_data = {
            'walls': [],
            'player': {'x': player[0], 'y': player[1]}
            }
    for wall in walls:
        scene_data['walls'].append({'x': wall.x, 'y': wall.y})
    with open(file_name, 'w') as file:
        file.write(json.dumps(scene_data))
        file.close()

    print('Done!\n')
    return True


def load_scene(walls, player):
    print('Loading...')

    print('Input file name: ')
    file_name = ''
    run = True
    while run:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                return False
            if e.type == pg.KEYDOWN:
                if e.key == pg.K_RETURN:
                    run = False
                elif e.key == pg.K_ESCAPE:
                    print('Canceled!\n')
                    return True
                elif e.key == pg.K_BACKSPACE:
                    file_name = file_name[:-1]
                else:
                    file_name += e.unicode
        pg.display.get_surface().fill(black, (0, 0, 512, 55))
        draw_text('Load: ' + file_name, 10, 10)
        pg.display.flip()
    print()
    if not len(file_name):
        file_name = 'level.json'

    with open(file_name, 'r') as file:
        scene_data = json.loads(file.read())
        file.close()

    walls.clear()
    for wall in scene_data['walls']:
        walls.append(Wall(wall['x'], wall['y']))
    player[0] = scene_data['player']['x']
    player[1] = scene_data['player']['y']

    print('Done!\n')
    return True


def main():
    SCREEN = (512, 448)
    screen = pg.display.set_mode(SCREEN)
    pg.display.set_caption('Level Editor')
    pg.key.set_repeat(250, 50)
    
    walls = []
    sprite = []
    cur_sprite = 0
    sprite.append(pg.image.load('./art/brick.png').convert())
    sprite.append(pg.image.load('./art/player.png').convert().subsurface((0, 0, 8, 12)))

    scene_x = 0
    scene_y = 0

    player = [0, 4]

    running = True
    while running:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                running = False
            if e.type == pg.MOUSEWHEEL:
                if e.y == 1:
                    cur_sprite -= 1
                    if cur_sprite < 0: cur_sprite = len(sprite) - 1
                elif e.y == -1:
                    cur_sprite += 1
                    if cur_sprite > len(sprite) - 1: cur_sprite = 0
            if e.type == pg.KEYDOWN:
                # Clear all
                if e.key == pg.K_c:
                    walls.clear()
                # Rest camera to origin
                if e.key == pg.K_o:
                    scene_x = 0
                    scene_y = 0
                # Delete most recent block
                if e.key == pg.K_u and len(walls) > 0:
                    walls.pop()
                # Save scene data
                if e.key == pg.K_k:
                    if not save_scene(walls, player):
                        running = False
                # Load scene data
                if e.key == pg.K_l:
                    if not load_scene(walls, player):
                        running = False
                # Cycle next sprite
                if e.key == pg.K_e:
                    cur_sprite += 1
                    if cur_sprite > len(sprite) - 1: cur_sprite = 0
                # Cycle prev sprite
                if e.key == pg.K_q:
                    cur_sprite -= 1
                    if cur_sprite < 0: cur_sprite = len(sprite) - 1

                # Camera controls
                if e.key == pg.K_a:
                    scene_x += 8
                elif e.key == pg.K_d:
                    scene_x -= 8
                elif e.key == pg.K_w:
                    scene_y += 8
                elif e.key == pg.K_s:
                    scene_y -= 8

        if pg.mouse.get_pressed()[0]:
            pos = pg.mouse.get_pos()
            x = (pos[0] / 2 // 8) * 8 - scene_x
            y = (pos[1] / 2 // 8) * 8 - scene_y
            if cur_sprite == 0:
                test = True
                for wall in walls:
                    if wall.x == x and wall.y == y:
                        test = False
                if test:
                    walls.append(Wall(x, y))
            elif cur_sprite == 1:
                player = [x, y + 4]
        elif pg.mouse.get_pressed()[2]:
            pos = pg.mouse.get_pos()
            x = (pos[0] / 2 // 8) * 8 - scene_x
            y = (pos[1] / 2 // 8) * 8 - scene_y
            for i, wall in enumerate(walls):
                if wall.x == x and wall.y == y:
                    walls.pop(i)

        screen_buffer = pg.Surface((256, 224))

        # Draw walls
        for wall in walls:
            screen_buffer.blit(sprite[0], (wall.x + scene_x, wall.y + scene_y))
        # Draw player
        screen_buffer.blit(sprite[1], (player[0] + scene_x, player[1] + scene_y))

        # Draw cursor
        rect = sprite[cur_sprite].get_rect()
        if cur_sprite == 1: rect.h = 16
        cursor = pg.Surface((rect.w, rect.h))
        cursor.fill((255, 0, 0))
        cursor.set_alpha(100)
        pos = pg.mouse.get_pos()
        x = (pos[0] / 2 // 8) * 8
        y = (pos[1] / 2 // 8) * 8
        screen_buffer.blit(cursor, (x, y))

        screen.blit(pg.transform.scale(screen_buffer, SCREEN), (0, 0))
        pg.display.update()


if __name__ == '__main__':
    main()
    pg.quit()
    quit()
