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


def save_scene(walls):
    print('Saving...')

    print('Input file name: ')
    file_name = ''
    run = True
    while run:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                return False
            if e.type == pg.KEYDOWN:
                if e.key == pg.K_RETURN and len(file_name):
                    run = False
                elif e.key == pg.K_BACKSPACE:
                    file_name = file_name[:-1]
                else:
                    file_name += e.unicode
        pg.display.get_surface().fill(black, (0, 0, 512, 55))
        draw_text('Save: ' + file_name, 10, 10)
        pg.display.flip()
    print()

    scene_data = {
            'walls': []
            }
    for wall in walls:
        scene_data['walls'].append({'x': wall.x, 'y': wall.y})
    with open(file_name, 'w') as file:
        file.write(json.dumps(scene_data))
        file.close()

    print('Done!\n')
    return True


def load_scene(walls):
    print('Loading...')

    print('Input file name: ')
    file_name = ''
    run = True
    while run:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                return False
            if e.type == pg.KEYDOWN:
                if e.key == pg.K_RETURN and len(file_name):
                    run = False
                elif e.key == pg.K_BACKSPACE:
                    file_name = file_name[:-1]
                else:
                    file_name += e.unicode
        pg.display.get_surface().fill(black, (0, 0, 512, 55))
        draw_text('Load: ' + file_name, 10, 10)
        pg.display.flip()
    print()

    with open(file_name, 'r') as file:
        scene_data = json.loads(file.read())
        file.close()

    walls.clear()
    for wall in scene_data['walls']:
        walls.append(Wall(wall['x'], wall['y']))

    print('Done!\n')
    return True


def main():
    SCREEN = (512, 448)
    screen = pg.display.set_mode(SCREEN)
    pg.key.set_repeat(250, 50)
    
    walls = []
    sprite = pg.image.load('../art/brick.png')

    scene_x = 0
    scene_y = 0

    running = True
    while running:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                running = False
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
                    if not save_scene(walls):
                        running = False
                # Load scene data
                if e.key == pg.K_l:
                    if not load_scene(walls):
                        running = False

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
            test = True
            for wall in walls:
                if wall.x == x and wall.y == y:
                    test = False
            if test:
                walls.append(Wall(x, y))
        elif pg.mouse.get_pressed()[2]:
            pos = pg.mouse.get_pos()
            x = (pos[0] / 2 // 8) * 8 - scene_x
            y = (pos[1] / 2 // 8) * 8 - scene_y
            for i, wall in enumerate(walls):
                if wall.x == x and wall.y == y:
                    walls.pop(i)

        screen_buffer = pg.Surface((256, 224))

        for wall in walls:
            screen_buffer.blit(sprite, (wall.x + scene_x, wall.y + scene_y))

        screen.blit(pg.transform.scale(screen_buffer, SCREEN), (0, 0))
        pg.display.update()


if __name__ == '__main__':
    main()
    pg.quit()
    quit()
