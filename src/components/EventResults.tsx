import { EOL } from 'os';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  DriverGroup,
  DriverId,
  LongCarClass,
  ShortCarClass,
  TimeSelection,
  to_display_name,
} from 'rusty/rusty';
import {
  ClassResults,
  Driver,
  EventResults as EventResultsData,
} from '../services/rust_helpers';
import { calculatePointsForDriver, calculateTrophies } from '../services';
import { RamDownload } from './DownloadButton';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  results?: EventResultsData;
  ladiesIds?: DriverId[]; // IDs of Ladies drivers from championship results
}

interface EventResultsState {
  csvContent?: string;
  exportFilename?: string;
}

export class EventResults extends Component<
  EventResultsProps,
  EventResultsState
> {
  private static readonly MAX_LAP_COUNT = 10;

  constructor(props: Readonly<EventResultsProps>) {
    super(props);
    this.state = {};
  }

  public render() {
    if (this.props.results) {
      return [
        <Row key={0} className={'top-buffer'}>
          <Col>
            <h2>Event Results</h2>

            <Accordion>
              {this.displayClassResults()}

              {this.displayCombinedResults(DriverGroup.PAX)}

              {this.displayCombinedResults(DriverGroup.Raw)}

              {this.displayCombinedResults(DriverGroup.Novice)}

              {this.props.ladiesIds
                ? this.displayCombinedResults(DriverGroup.Ladies)
                : null}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.exportFilename}
          content={this.state.csvContent}
          contentType={'text/csv'}
          downloadComplete={() =>
            this.setState({ csvContent: undefined, exportFilename: undefined })
          }
        />,
      ];
    } else {
      return null;
    }
  }

  private displayClassResults(): JSX.Element {
    return (
      <Card>
        <Card.Header key={'class'}>
          <Accordion.Toggle eventKey={'class'} as={Button} variant={'link'}>
            Results by Class
          </Accordion.Toggle>
          <Button variant={'secondary'}>
            <FontAwesomeIcon
              className={'clickable'}
              icon={faDownload}
              onClick={() => this.exportResultsByClassCsv()}
            />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={'class'}>
          <Card.Body>
            <Accordion>
              {this.props.results?.get_all().map((classResults, index) => (
                <Card key={index}>
                  <Card.Header>
                    <Accordion.Toggle
                      eventKey={`${index}`}
                      as={Button}
                      variant={'link'}
                    >
                      {to_display_name(classResults.car_class.long)}
                    </Accordion.Toggle>
                  </Card.Header>

                  <Accordion.Collapse eventKey={`${index}`}>
                    <Card.Body>
                      <Table key={index} striped hover borderless>
                        <thead>
                          <tr>
                            <th>Tr</th>
                            <th>RK</th>
                            <th>Pos</th>
                            <th>Nbr</th>
                            <th>Name</th>
                            <th>Car</th>
                            <th>Region</th>
                            <th colSpan={EventResults.MAX_LAP_COUNT}>
                              Lap Times
                            </th>
                            <th>Best Lap</th>
                            <th>Difference</th>
                          </tr>
                        </thead>
                        <tbody>
                          {this.displayClassResultsRows(classResults)}
                        </tbody>
                      </Table>
                    </Card.Body>
                  </Accordion.Collapse>
                </Card>
              ))}
            </Accordion>
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private displayClassResultsRows(classResults: ClassResults): JSX.Element[] {
    const bestTimeOfDay = classResults.get_best_in_class();
    return classResults.drivers.map((driver, index) => {
      return (
        <tr key={index}>
          <td>{index < classResults.trophy_count ? 'T' : ''}</td>
          <td>{driver.rookie ? 'R' : ''}</td>
          <td>{driver.position}</td>
          <td>{driver.car_number}</td>
          <td>{driver.get_name()}</td>
          <td>{driver.get_car_description()}</td>
          <td>{driver.get_region()}</td>
          {driver.day_1_times!.map((lapTime, index) => (
            <td key={`day1Time-${index}`}>{lapTime.toString()}</td>
          ))}
          {new Array(EventResults.MAX_LAP_COUNT - driver.day_1_times!.length)
            .fill(null)
            .map((_, index) => (
              <td key={`day1TimeFiller-${index}`} />
            ))}
          <td>{driver.best_lap().toString()}</td>
          <td>{driver.difference(bestTimeOfDay)}</td>
        </tr>
      );
    });
  }

  private displayCombinedResults(driverGroup: DriverGroup): JSX.Element {
    const sortedDrivers = this.props.results!.get_drivers(driverGroup);
    const fastestDriver = sortedDrivers[0];
    const fastestOfDay = fastestDriver
      .best_lap()
      .with_pax(
        driverGroup === DriverGroup.Raw ? 1 : fastestDriver.pax_multiplier,
      );
    const trophyCount = calculateTrophies(sortedDrivers);
    return (
      <Card>
        <Card.Header key={driverGroup}>
          <Accordion.Toggle
            eventKey={DriverGroup[driverGroup]}
            as={Button}
            variant={'link'}
          >
            {DriverGroup[driverGroup]} Results
          </Accordion.Toggle>
          <Button
            variant={'secondary'}
            onClick={() =>
              this.exportCombinedResultsToCsv(
                sortedDrivers,
                fastestOfDay,
                driverGroup,
              )
            }
          >
            <FontAwesomeIcon className={'clickable'} icon={faDownload} />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={DriverGroup[driverGroup]}>
          <Card.Body>
            <Table striped hover borderless>
              <thead>
                <tr>
                  <th>Trophy</th>
                  <th>Position</th>
                  <th>Class</th>
                  <th>Car #</th>
                  <th>Name</th>
                  <th>Car</th>
                  <th>Region</th>
                  <th>{driverGroup ? 'PAX' : 'Raw Corrected'} Time</th>
                  <th>Difference</th>
                </tr>
              </thead>

              <tbody>
                {sortedDrivers.map((driver, index) => (
                  <tr key={index}>
                    <td>{index < trophyCount ? 'T' : ''}</td>
                    <td>{index + 1}</td>
                    <td>{driver.car_class.short}</td>
                    <td>{driver.car_number}</td>
                    <td>{driver.get_name()}</td>
                    <td>{driver.get_car_description()}</td>
                    <td>{driver.get_region()}</td>
                    <td>
                      {driver
                        .best_lap()
                        .to_string(
                          driverGroup === DriverGroup.Raw
                            ? undefined
                            : driver.pax_multiplier,
                        )}
                    </td>
                    <td>
                      {driver.difference(
                        fastestOfDay,
                        driverGroup !== DriverGroup.Raw,
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private exportResultsByClassCsv() {
    const lines = [
      [
        'Trophy',
        'Pos',
        'Name',
        'Car',
        'Class',
        'Number',
        'Total Time',
        'Index Time',
        'From Previous',
        'From Top',
        'Points',
        'Region',
      ],
      ...Object.values(this.props.results!)
        .map((classResults) => this.exportClassResultsToCsv(classResults))
        .flat(),
    ];
    this.setState({
      exportFilename: 'event_results.csv',
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }

  private exportClassResultsToCsv(classResults: ClassResults): string[][] {
    const bestLapInClass = classResults.get_best_in_class();
    const bestIndexTime =
      (bestLapInClass || Infinity) * classResults.drivers[0].pax_multiplier;
    return [
      [
        `${ShortCarClass[classResults.car_class.short]} - ${to_display_name(
          classResults.car_class.long,
        )}`,
      ],
      ...classResults.drivers.map((driver, index) => {
        return [
          index < classResults.trophy_count ? 'T' : '',
          `${driver.position}`,
          driver.get_name(),
          driver.get_car_description(),
          ShortCarClass[classResults.car_class.short],
          `${driver.car_number}`,
          driver.best_lap().to_string(undefined, false),
          driver.best_lap().to_string(driver.pax_multiplier, false),
          index === 0
            ? ''
            : driver.difference(
                (classResults.drivers[index - 1].best_lap().time || Infinity) *
                  driver.pax_multiplier,
                true,
              ),
          driver.difference(bestIndexTime, true),
          `${calculatePointsForDriver(
            bestIndexTime,
            driver,
            driver.pax_multiplier,
          )}`,
          driver.get_region(),
        ];
      }),
    ];
  }

  private exportCombinedResultsToCsv(
    sortedDrivers: Driver[],
    fastestOfDay: number,
    driverGroup: DriverGroup,
  ): void {
    const isRawTime = driverGroup === DriverGroup.Raw;
    const trophyCount = calculateTrophies(sortedDrivers);
    const results = [
      [
        'Trophy',
        'Position',
        'Name',
        'Car',
        'Class',
        'Car #',
        `${isRawTime ? 'Best' : 'Index'} Time`,
        'From Previous',
        'From Top',
        ...(isRawTime ? [] : ['Points']),
      ],
      ...sortedDrivers.map((driver, index) => {
        const previousDriver = index === 0 ? driver : sortedDrivers[index - 1];
        return [
          index < trophyCount ? 'T' : '',
          `${index + 1}`,
          driver.get_name(),
          driver.get_car_description(),
          driver.car_class.short,
          `${driver.car_number}`,
          driver
            .best_lap(TimeSelection.Day1)
            .to_string(isRawTime ? undefined : driver.pax_multiplier, false),
          index === 0
            ? ''
            : driver.difference(
                (previousDriver.best_lap().time || Infinity) *
                  (isRawTime ? 1 : previousDriver.pax_multiplier),
                !isRawTime,
              ),
          driver.difference(fastestOfDay, !isRawTime),
          ...(isRawTime
            ? []
            : [
                calculatePointsForDriver(
                  fastestOfDay,
                  driver,
                  driver.pax_multiplier,
                ),
              ]),
        ];
      }),
    ];
    this.setState({
      exportFilename: `event_${DriverGroup[
        driverGroup
      ].toLowerCase()}_results.csv`,
      csvContent: results.map((row) => `"${row.join('","')}"`).join(EOL),
    });
  }

  /*private exportFullResultsToCsv(): void {
        const lines = (
          Object.entries(this.props.results!) as [ShortCarClass, ClassResults][]
        )
          .map(([carClass, classResults]) => {
            const bestInClass = classResults.drivers[0].bestLap().time;
            return [
              [`${carClass} (Trophies: ${classResults.trophyCount})`],
              [
                'RK',
                'Pos',
                'Nbr',
                "Driver's name, Town",
                'Car, Sponsor',
                'Region',
                'Times',
                ...new Array(EventResults.MAX_LAP_COUNT - 1)
                  .fill(null)
                  .map(() => ''),
                'Best Lap',
                'Difference',
                'From Previous',
              ],
              ...classResults.drivers.map((driver) => {
                const previousDriver = classResults.drivers[driver.position! - 2];
                return [
                  driver.rookie ? 'R' : '',
                  `${driver.position}`,
                  driver.carNumber,
                  driver.name,
                  driver.carDescription,
                  driver.region,
                  ...driver.getTimes('day1')!,
                  ...new Array(
                    EventResults.MAX_LAP_COUNT - driver.getTimes('day1')!.length,
                  )
                    .fill(null)
                    .map(() => ''),
                  driver.bestLap(),
                  driver.position === 1 ? '' : driver.difference(bestInClass),
                  driver.position === 1
                    ? ''
                    : driver.difference(previousDriver.bestLap().time),
                ];
              }),
            ];
          })
          .flat();
        this.setState({
          exportFilename: 'event_results.csv',
          csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
        });
      }*/
}
